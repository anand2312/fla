<script>

    
    const modes = ["state", "transition", "erase"];
    let currMode = modes[0];
    
    /* Determines current mode of the pointer */
    function modeSet(event) {
        currMode = event.target.id;
    }
    
    const shortcutMap = new Map();
    
    let transitionIdx = -1;

    // shortcuts mapping
    shortcutMap.set("1", modes[0]);
    shortcutMap.set("2", modes[1]);
    shortcutMap.set("3", modes[2]);

    function shortcuts(event) {
        let key = String.fromCharCode(event.keyCode);

        if (key !== "2") {
            transitionIdx = -1;
        }

        currMode = shortcutMap.get(key);
    }

    let circles = [];
    let radius = 50;

    function handleClick(event) {

        switch (currMode) {

            case "transition":

                /*

                    The idea here is that:

                    1) The state can have several "attachment points" where transition 
                    lines can connect. For now it will be limited to some constant, eg: 8,
                    like so:

                               \ | /
                               - 0 -
                               / | \
                    
                    Later this control could be given to the user. 

                    2) A choice is to be made between bezier curves, straight lines or some
                    combination of them for the appearance of the transitions

                    3) Connected Transitions must conform to movement of states

                    4) Implementing tools to edit transitions

                    5) Couple it with an input field for the transition
                    
                */

                break;
            
            /* Removing (for now, only) states clicked on from the board */
            case "erase":
                if (event.target.tagName !== "circle") {
                    return;
                }

                let idx = Number(event.target.id);
                circles = circles.slice(0, idx).concat(circles.slice(idx+1, circles.length));

                break;
        }
    
    }

    let points = 6;
    let highlightingCoords = [];
    
    function calculateRelativeCoords() {
        
        // x = r * cos(θ), y = r * sin(θ)
        for (let i = 0; i < points; i++) {

            let theta = i * 2 * Math.PI / points;
            const coords = {
                x: radius * Math.cos(theta),
                y: radius * Math.sin(theta)
            }
            
            highlightingCoords = highlightingCoords.concat(coords);
        }
    }
    
    
    let movingIdx = -1;
    
    // Relative location of click from the center of the circle
    let clickX = 0, clickY = 0;         // Variables for cursor

    // temporary variable               // Variables for transition
    let ptRadius = 7;

    // Sidenote, erase might function like a brush later
    // so it will be in the drag function
    function handleMovement(event) {

        switch (currMode) {

            /* When the cursor is moving over the svg tag and the 
               mouse is held down on a circle, it's position is updated */
            case "state":
                
                if (movingIdx < 0) {
                    return;
                }

                let mouseX = event.clientX - event.target.parentNode.getBoundingClientRect().left;
                let mouseY = event.clientY - event.target.parentNode.getBoundingClientRect().top;
        
                circles[movingIdx].cx = mouseX + clickX;
                circles[movingIdx].cy = mouseY + clickY;

                break;
            
            case "transition":

                if (event.target.tagName !== "circle") {
                    transitionIdx = -1;
                    return;
                }

                if (points !== highlightingCoords.length) {
                    calculateRelativeCoords();
                }
                
                if (event.target.classList.contains("attach-pt") ||
                    (transitionIdx >= 0 && transitionIdx == Number(event.target.id))) {
                    return;
                }

                transitionIdx = Number(event.target.id);

        }
    }

    
    /* Determines if mouse is clicked and which circle is to be moved */
    function setMovement(event) {

        if (currMode !== "state") {
            return;
        }

        switch (event.type) {
            case "mousedown":

                if (event.target.tagName !== "circle") {

                    // Position of the mouse relative to the board
                    const circle = {
                        cx: event.clientX - event.target.getBoundingClientRect().left,
                        cy: event.clientY - event.target.getBoundingClientRect().top
                    };
                    circles = circles.concat(circle);
                    
                    return;
                }
                
                movingIdx = Number(event.target.id);

                // Putting circle clicked on to the top
                let circleCpy = circles[movingIdx];
                circles = circles.slice(0, movingIdx).concat(circles.slice(movingIdx+1, circles.length));
                circles = circles.concat(circleCpy);
                movingIdx = circles.length - 1;

                // circle coords - (click coords) = distance of click from center of circle
                clickX = event.target.cx.animVal.value - 
                        (event.clientX - event.target.parentNode.getBoundingClientRect().left);
                clickY = event.target.cy.animVal.value -
                        (event.clientY - event.target.parentNode.getBoundingClientRect().top);

                break;
            
            case "mouseup":
                movingIdx = -1;
                clickX = clickY = 0;

                break;
        }
    }
    
</script>

<svelte:window on:keydown={shortcuts}/>

<div class="board">
    <div class="modes">
        {#each modes as mode}
            {#if currMode === mode}
                <div id="{mode}" class="clicked"  on:click={modeSet}>{mode}</div>
            {:else}
                <div id="{mode}" on:click={modeSet}>{mode}</div>
            {/if}
        {/each}
    </div>
    <svg class="canvas" on:mousedown={setMovement} on:mousemove={handleMovement} on:mouseup={setMovement} 
                        on:click={handleClick}>
        {#each circles as circ, index}
            <circle cx={circ.cx} cy={circ.cy} r={radius} />
            <circle id={index.toString()} class="bounding" cx={circ.cx} cy={circ.cy} r={radius+ptRadius} />
        {/each}
        {#if transitionIdx >= 0}
            {#each highlightingCoords as coords}
                <circle class="attach-pt" cx={circles[transitionIdx].cx + coords.x}
                        cy={circles[transitionIdx].cy + coords.y} r={ptRadius} />
            {/each}
        {/if}
    </svg>
</div>

<style>
    .board {
        width: 100%;
        background-color:  #274c43;
        border: black solid 5px;
        border-radius: 1em;
    }

    .modes {
        border: white dotted 1px;
        width: 6.5em;
        color: beige;
        position: absolute;
    }

    .modes > div {
        padding: 1em;
    }

    .clicked {
        background-color: rgba(0, 0, 0, 0.4);
    }

    svg {
        width: 100%;
        height: 40em;
    }

    circle {
        stroke: black;
        stroke-width: 5px;
        fill: rgba(248, 243, 194, 0.851);
    }

    .bounding {
        stroke: white;
        stroke-width: 1px;
        stroke-dasharray: 10 5;
        fill: rgba(0, 0, 0, 0);
    }

    .attach-pt {
        fill: rgba(0, 255, 0, 0.4);
        stroke: none;
    }

    .attach-pt:hover {
        fill: rgba(0, 255, 0, 1);
    }

</style>