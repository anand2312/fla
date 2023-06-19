<script>

    
    const modes = ["state", "transition", "erase"];
    let currMode = modes[0];
    
    /* Determines current mode of the pointer */
    function modeSet(event) {
        currMode = event.target.id;
    }
    
    const shortcutMap = new Map();
    
    let transitionIdx = -1;
    let transitions = [];
    //     [{x: 20, y: 300}, {x: 200, y: 30}, {x: 400, y: 50}, {x: 200, y: 600}]
    // ];

    // shortcuts mapping
    shortcutMap.set("1", modes[0]);
    shortcutMap.set("2", modes[1]);
    shortcutMap.set("3", modes[2]);

    function shortcuts(event) {
        let key = String.fromCharCode(event.keyCode);

        if (key !== "2" && Array.from(shortcutMap.keys()).includes(key)) {
            transitionIdx = -1;
        }
        if (Array.from(shortcutMap.keys()).includes(key)) {
            currMode = shortcutMap.get(key);
        }
    }

    let circles = [];
    let radius = 50;

    function handleClick(event) {

        switch (currMode) {

            /* case "transition":

                

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
                    
                

                break;
            */
            
            
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
                x: radius * Math.cos(theta - Math.PI/2),
                y: radius * Math.sin(theta - Math.PI/2)
            }
            
            highlightingCoords = highlightingCoords.concat(coords);
        }
    }
    
    
    let movingIdx = -1;
    
    // Relative location of click from the center of the circle
    let clickX = 0, clickY = 0;         // Variables for cursor

    // temporary variable               // Variables for transition
    let ptRadius = 7;

    let attached = -1;

    let tempTransition = []

    // Sidenote, erase might function like a brush later
    // so it will be in this function
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

                if (tempTransition.length > 0) {
                    tempTransition[1] = {
                        x: event.clientX - event.target.parentNode.getBoundingClientRect().left,
                        y: event.clientY - event.target.parentNode.getBoundingClientRect().top
                    }
                }
                    
                if (event.target.tagName !== "circle") {
                    if ( attached >= 0) {
                        return;
                    }
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

                break;

        }
    }

    
    /* Determines if mouse is clicked and which circle is to be moved */
    function setMovement(event) {

        if (event.type === "mousedown") {
            switch (currMode) {
                case "state":
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
                    clickX = Number(event.target.attributes.cx.value) - 
                        (event.clientX - event.target.parentNode.getBoundingClientRect().left);
                    clickY = Number(event.target.attributes.cy.value) -
                        (event.clientY - event.target.parentNode.getBoundingClientRect().top);

                    break;
                
                case "transition":

                    if (!event.target.classList.contains("attach-pt")) {
                        return;
                    }
                    
                    let attID = event.target.id;
                    let mouseX = event.clientX - event.target.parentNode.getBoundingClientRect().left;
                    let mouseY = event.clientY - event.target.parentNode.getBoundingClientRect().top;

                    attached = Number(attID.slice(3, attID.length));
                    tempTransition = [
                        {x: Number(event.target.attributes.cx.value), y: Number(event.target.attributes.cy.value)},
                        {x: mouseX, y: mouseY}
                    ];

                    break;
            }
        } else if (event.type === "mouseup") {
            switch (currMode) {
                case "state":
                    movingIdx = -1;
                    clickX = clickY = 0;

                    break;
                
                case "transition":
                    
                    attached = -1;

                    if (event.target.classList.contains("attach-pt")) {
                        
                        console.log("before adding transition");
                        const newTransition = [
                            {x: tempTransition[0].x, y: tempTransition[0].y},
                            {x: 150, y: 150},
                            {x: 150, y: 150},
                            {x: Number(event.target.attributes.cx.value), y: Number(event.target.attributes.cy.value)}
                        ];
                        
                        transitions.push(newTransition);
                        transitions = transitions;
                    }

                    tempTransition = [];

                    break;
            }
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
        {#if tempTransition.length > 0}
            <path D={"M "+tempTransition[0].x+","+tempTransition[0].y+" L "+tempTransition[1].x+","+tempTransition[1].y} />
        {/if}
        {#each circles as circ, index}
            <circle cx={circ.cx} cy={circ.cy} r={radius} />
            <circle id={index.toString()} class="bounding" cx={circ.cx} cy={circ.cy} r={radius+ptRadius} />

            {#if index === transitionIdx} 
                {#each highlightingCoords as coords, index}
                    <circle class="attach-pt" id={"idx"+index} cx={circles[transitionIdx].cx + coords.x}
                            cy={circles[transitionIdx].cy + coords.y} r={ptRadius}/>
                {/each}
            {/if}
        {/each}
        {#each transitions as t, index}
            <path D={"M "+t[0].x+","+t[0].y + " C "+t[1].x+","+t[1].y+" "+t[2].x+","+t[2].y+" "+t[3].x+","+t[3].y}
            class={"tx-"+index}/>
        {/each}
        {#each transitions as trans}
            {#each trans as pts}
                <circle cx={pts.x} cy={pts.y} r={4} />
            {/each}
        {/each}
    </svg>
</div>


{#if currMode === modes[0]} 
    <style>
        circle {
            cursor: move;
        }
    </style>
{/if}

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
        fill: rgba(0, 255, 0, 0.6);
        stroke: none;
    }

    .attach-pt:hover {
        fill: rgba(0, 255, 0, 1);
    }

    path {
        fill: none;
        stroke: black;
        stroke-width: 4px;
    }

</style>