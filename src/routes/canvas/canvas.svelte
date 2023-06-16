<script>

    const modes = ["cursor", "state", "transition", "erase"];
    let currMode = modes[1];

    /* Determines current mode of the pointer */
    function modeSet(event) {
        currMode = event.target.id;
    }


    function getCircleIdx(xval = 0, yval = 0) {
        for (let i = 0; i < circles.length; i++) {
            if (Math.round(xval) == Math.round(circles[i].cx) &&
                Math.round(yval) == Math.round(circles[i].cy)) {
                return i;
            }
        }
    }

    let circles = [];
    let radius = 50;

    function handleClick(event) {

        switch (currMode) {

            /* Handles adding states to the canvas */
            case "state":

                if (!event.target.classList.contains("canvas")) {
                    return;
                }

                // Position of the mouse relative to the board
                const circle = {
                    cx: event.clientX - event.target.getBoundingClientRect().left,
                    cy: event.clientY - event.target.getBoundingClientRect().top
                };
                circles = circles.concat(circle);
                break;

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
                // Target circle's x and y values
                let circX = event.target.cx.animVal.value;
                let circY = event.target.cy.animVal.value;

                let idx = getCircleIdx(circX, circY);
                circles = circles.slice(0, idx).concat(circles.slice(idx+1, circles.length));

                break;
        }

    }

    let movingIdx = -1;

    // Relative location of click from the center of the circle
    let clickX = 0, clickY = 0;

    // Sidenote, erase might function like a brush later
    // so it will be in the drag function
    function handleDrag(event) {

        switch (currMode) {

            /* When the cursor is moving over the svg tag and the 
               mouse is held down on a circle, it's position is updated */
            case "cursor":
                
                if (movingIdx < 0) {
                    return;
                }

                let mouseX = event.clientX - event.target.parentNode.getBoundingClientRect().left;
                let mouseY = event.clientY - event.target.parentNode.getBoundingClientRect().top;
        
                circles[movingIdx].cx = mouseX + clickX;
                circles[movingIdx].cy = mouseY + clickY;

                break;
        }
    }

    /* Determines if mouse is clicked and which circle is to be moved */
    function setMovement(event) {

        if (currMode !== "cursor") {
            return;
        }

        if (event.target.tagName !== "circle") {
            return;
        }
        movingIdx = -1;
        clickX = clickY = 0;
        
        if (event.type !== "mousedown") {
            return;
        }
        movingIdx = getCircleIdx(event.target.cx.animVal.value, event.target.cy.animVal.value);

        let circleCpy = circles[movingIdx];
        circles = circles.slice(0, movingIdx).concat(circles.slice(movingIdx+1, circles.length));
        circles = circles.concat(circleCpy);
        movingIdx = circles.length - 1;

        // circle coords - (click coords) = distance of click from center of circle
        clickX = event.target.cx.animVal.value - 
                 (event.clientX - event.target.parentNode.getBoundingClientRect().left);
        clickY = event.target.cy.animVal.value -
                 (event.clientY - event.target.parentNode.getBoundingClientRect().top);

    }

</script>

<div class="board">
    <div class="modes">
        {#each modes as mode}
            {#if currMode === mode}
                <div id="{mode}" class="mbtn clicked"  on:click={modeSet}>{mode}</div>
            {:else}
                <div id="{mode}" class="mbtn" on:click={modeSet}>{mode}</div>
            {/if}
        {/each}
    </div>
    <svg class="canvas" on:mousedown={setMovement} on:mousemove={handleDrag} on:mouseup={setMovement} on:click={handleClick}>
        {#each circles as circ}
            <circle cx={circ.cx} cy={circ.cy} r={radius}/>
        {/each}
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
</style>