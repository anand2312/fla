<script>

    let currMode = "cursor";
    let prev = "placeholder";

    /* Determines current mode of the pointer */
    function mode(event) {
        if (event.target === prev) { return; }
        
        currMode = event.target.id;
        event.target.classList.add("clicked");
        if (typeof(prev) === 'object') {
            prev.classList.remove("clicked");
        } else {
            event.target.parentElement.firstChild.classList.remove("clicked");
        }

        prev = event.target;
    }

    let circles = [];
    let radius = 50;

    function handleClick(event) {

        if (!event.target.classList.contains("canvas")) {
            return;
        }

        switch (currMode) {
            case "cursor":

                break;

            /* Handles adding states to the canvas */
            case "state":
                // Position of the mouse relative to the board
                const circle = {
                    cx: event.clientX - event.target.getBoundingClientRect().left,
                    cy: event.clientY - event.target.getBoundingClientRect().top
                };
                circles = circles.concat(circle);
                break;

            case "transition":

                break;

            case "erase":

                break;
        }

    }
</script>

<div class="board">
    <div class="modes">
        <div id="cursor" class="mbtn clicked" on:click={mode}>cursor</div>
        <div id="state" class="mbtn" on:click={mode}>state</div>
        <div id="transition" class="mbtn" on:click={mode}>transition</div>
        <div id="erase" class="mbtn" on:click={mode}>erase</div>
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
        height: 50em;
        /* position: absolute; */
    }

    circle {
        /* border: black solid 5px; */
        stroke: black;
        stroke-width: 5px;
        fill: rgba(248, 243, 194, 0.851);
    }
</style>