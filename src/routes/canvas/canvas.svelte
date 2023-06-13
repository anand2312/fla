<script>

    let currMode = "cursor";

    /* Determines current mode of the pointer */
    function mode(event) {
        currMode = event.target.id;
    }

    let circles = [];
    let radius = 50;

    function handleClick(event) {

        switch (currMode) {
            case "cursor":

                break;

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
                console.log(event.target);
                if (event.target.tagName !== "circle") {
                    return;
                }
                // Target circle's x and y values
                let circX = event.target.cx.animVal.value;
                let circY = event.target.cy.animVal.value;

                for (let i = 0; i < circles.length; i++) {
                    if (Math.round(circX) == Math.round(circles[i].cx) &&
                        Math.round(circY) == Math.round(circles[i].cy)) {
                        circles = circles.slice(0, i).concat(circles.slice(i+1, circles.length));
                        return;
                    }
                }

                break;
        }

    }
</script>

<div class="board">
    <div class="modes">
        {#if currMode === "cursor"}
            <div id="cursor" class="mbtn clicked" on:click={mode}>cursor</div>
        {:else}
            <div id="cursor" class="mbtn" on:click={mode}>cursor</div>
        {/if}
        {#if currMode === "state"}
            <div id="state" class="mbtn clicked" on:click={mode}>state</div>
        {:else}
            <div id="state" class="mbtn" on:click={mode}>state</div>
        {/if}
        {#if currMode === "transition"}
            <div id="transition" class="mbtn clicked" on:click={mode}>transition</div>
        {:else}
            <div id="transition" class="mbtn" on:click={mode}>transition</div>
        {/if}
        {#if currMode === "erase"}
            <div id="erase" class="mbtn clicked" on:click={mode}>erase</div>
        {:else}
            <div id="erase" class="mbtn" on:click={mode}>erase</div>
        {/if}
    </div>
    <svg class="canvas" on:click={handleClick}>
        {#each circles as circ}
            <circle cx={circ.cx} cy={circ.cy} r={radius}/>
        {/each}
    </svg>
</div>

current mode is: {currMode}

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