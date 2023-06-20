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
                let id = event.target.id
                let idx = Number(id.slice(6, id.length));

                // let idx = Number(event.target.id);
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
    
    // if Pointer is attached to attachment point
    let attached = -1;
    
    // line to indicate new transition being dragged out
    let tempTransition = [];
    
    // path beginning from state number
    let pathFrom = -1;

    // All connected transitions of some state
    let connectedTx = [];

    // circle to display on top of the rest
    let circleOnTop = 0;
    
    /* Adds transitions connected to a state in the connectedTx array*/
    function getConnectedTransitions(circleIdx) {

        for (let i = 0; i < transitions.length; i++) {
            console.log
            if (circleIdx === transitions[i][4].from) {
                connectedTx.push({
                    index: i,
                    from: 1
                });
            }
            if (circleIdx === transitions[i][4].to) {
                connectedTx.push({
                    index: i,
                    from: 0
                });
            }
        }

        return connectedTx;
    }
    
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

                // Making transitions move along with states
                for (let i = 0; i < connectedTx.length; i++) {
                    if (connectedTx[i].from) {
                        transitions[connectedTx[i].index][0].x = 
                            mouseX + clickX + highlightingCoords[transitions[connectedTx[i].index][5].from].x;
                        transitions[connectedTx[i].index][0].y = 
                            mouseY + clickY + highlightingCoords[transitions[connectedTx[i].index][5].from].y;
                    } else {
                        transitions[connectedTx[i].index][3].x = 
                            mouseX + clickX + highlightingCoords[transitions[connectedTx[i].index][5].to].x;
                        transitions[connectedTx[i].index][3].y = 
                            mouseY + clickY + highlightingCoords[transitions[connectedTx[i].index][5].to].y;
                    }
                }

                break;
            
            case "transition":

                if (tempTransition.length > 0) {
                    tempTransition[1] = {
                        x: event.clientX - event.target.parentNode.getBoundingClientRect().left,
                        y: event.clientY - event.target.parentNode.getBoundingClientRect().top
                    };
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

                let id = event.target.id
                transitionIdx = Number(id.slice(6, id.length));

                break;

        }
    }

    
    /* Determines if mouse is clicked and sets appropriate variables according to the mode*/
    function setMovement(event) {
        switch (currMode) {
            case "state":

                // Here we add a new state
                if (event.target.tagName !== "circle") {

                    // Position of the mouse relative to the board
                    const circle = {
                        cx: event.clientX - event.target.getBoundingClientRect().left,
                        cy: event.clientY - event.target.getBoundingClientRect().top
                    };
                    circles = circles.concat(circle);

                    return;
                }

                // Here we move an existing state
                let id = event.target.id
                movingIdx = Number(id.slice(6, id.length));

                // Putting circle clicked on to the top
                circleOnTop = movingIdx;

                // circle coords - (click coords) = distance of click from center of circle
                clickX = Number(event.target.attributes.cx.value) - 
                    (event.clientX - event.target.parentNode.getBoundingClientRect().left);
                clickY = Number(event.target.attributes.cy.value) -
                    (event.clientY - event.target.parentNode.getBoundingClientRect().top);

                connectedTx = getConnectedTransitions(movingIdx);
                console.log(transitions);
                console.log(movingIdx);

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
                let circIdx = event.target.classList[1]; 
                pathFrom = Number(circIdx.slice(4, circIdx.length));

                break;
        }
    }

    /* Changes relevant variables when click is released according to the mode*/
    function unsetMovement(event) {
        switch (currMode) {
            case "state":
                movingIdx = -1;
                clickX = clickY = 0;
                connectedTx = [];

                break;
            
            case "transition":
                
                if (event.target.classList.contains("attach-pt")) {

                    let circIdx = event.target.classList[1];
                    let attachIdx = event.target.id;
                    
                    /*
                        0:   Start state atttachment point location
                        1,2: Bezier curve handles
                        3:   Destination state attachment point location
                        4:   From and to state number
                        5:   From and to attachment point index
                    */
                    const newTransition = [
                        {x: tempTransition[0].x, y: tempTransition[0].y},
                        {x: 150, y: 150},
                        {x: 150, y: 200},
                        {x: Number(event.target.attributes.cx.value), y: Number(event.target.attributes.cy.value)},
                        {from: pathFrom, to: Number(circIdx.slice(4, circIdx.length))},
                        {from: attached, to: Number(attachIdx.slice(3, attachIdx.length))}
                    ];
                    
                    transitions.push(newTransition);
                    transitions = transitions;
                }
                
                pathFrom = -1;
                attached = -1;
                tempTransition = [];

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
    <svg class="canvas" on:mousedown={setMovement} on:mousemove={handleMovement} on:mouseup={unsetMovement} 
                        on:click={handleClick}>
        {#if tempTransition.length > 0}
            <path D={"M "+tempTransition[0].x+","+tempTransition[0].y+" L "+tempTransition[1].x+","+tempTransition[1].y} />
        {/if}
        <!-- States -->
        {#each circles as circ, index}
            {#if index !== circleOnTop}
                <circle cx={circ.cx} cy={circ.cy} r={radius} />
                <circle id={"bound-"+index} class="bounding" cx={circ.cx} cy={circ.cy} r={radius+ptRadius} />

                {#if index === transitionIdx} 
                    {#each highlightingCoords as coords, idx}
                        <circle class={"attach-pt att-"+index} id={"idx"+idx} cx={circles[transitionIdx].cx + coords.x}
                                cy={circles[transitionIdx].cy + coords.y} r={ptRadius}/>
                    {/each}
                {/if}
            {/if}
        {/each}
        {#if circles.length > 0}
            <circle cx={circles[circleOnTop].cx} cy={circles[circleOnTop].cy} r={radius} />
            <circle id={"bound-"+circleOnTop} class="bounding" cx={circles[circleOnTop].cx} cy={circles[circleOnTop].cy} r={radius+ptRadius} />

            {#if circleOnTop === transitionIdx} 
                {#each highlightingCoords as coords, idx}
                    <circle class={"attach-pt att-"+circleOnTop} id={"idx"+idx} cx={circles[transitionIdx].cx + coords.x}
                            cy={circles[transitionIdx].cy + coords.y} r={ptRadius}/>
                {/each}
            {/if}
        {/if}
        <!-- Transitions -->
        {#each transitions as t, index}
            <path D={"M "+t[0].x+","+t[0].y + " C "+t[1].x+","+t[1].y+" "+t[2].x+","+t[2].y+" "+t[3].x+","+t[3].y}
                  id={"tx-"+index}/>
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