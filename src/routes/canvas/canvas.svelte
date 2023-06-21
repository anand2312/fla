<script lang="ts">

    interface Coordinates {
        x: number,
        y: number
    };

    interface Connection<T> {
        from: T,
        to: T
    };

    interface Transition {
        bezier: Array<Coordinates>,
        state: Connection<String>,
        attPt: Connection<number>
    };

    interface ConnectedTransition {
        index: number,
        from: number
    };

    interface StateProperties {
        coords: Coordinates,
        connTransitions: Array<ConnectedTransition>
    };

    const modes: Array<string> = ["state", "transition", "erase"];
    let currMode: String = modes[0];
    
    /* Determines current mode of the pointer */
    function modeSet(event: Event): void {
        currMode = (event.target as HTMLDivElement).id;
    }
    
    const shortcutMap = new Map<String, String>();
    
    let transitionIdx: number = -1;
    let transitionName: String = "";
    let transitions: Array<Transition> = [];

    let attachedCircle: String = "";
    
    // shortcuts mapping
    shortcutMap.set("1", modes[0]);
    shortcutMap.set("2", modes[1]);
    shortcutMap.set("3", modes[2]);

    function shortcuts(event: KeyboardEvent): void {

        if (event.key !== "2" && Array.from(shortcutMap.keys()).includes(event.key)) {
            transitionIdx = -1;

            transitionName = "";
        }
        if (Array.from(shortcutMap.keys()).includes(event.key)) {
            currMode = shortcutMap.get(event.key)!;
        }
    }

    // The name is temporary
    let circles: Array<String> = [];

    // state name (String) => state properties
    let states = new Map<String, StateProperties>();
    let radius: number = 50;

    function handleClick(event: MouseEvent): void {

        switch (currMode) {

            /* case "transition":

                

                    The idea here is that:

                    done: 1) The state can have several "attachment points" where transition 
                    lines can connect. For now it will be limited to some constant, eg: 8,
                    like so:

                               \ | /
                               - 0 -
                               / | \
                    
                    Later this control could be given to the user. 

                    done: 2) A choice is to be made between bezier curves, straight lines or some
                    combination of them for the appearance of the transitions

                    done: 3) Connected Transitions must conform to movement of states

                    4) Implementing tools to edit transitions

                    5) Couple it with an input field for the transition
                    
                

                break;
            */
            
            
            /* Removing (for now, only) states clicked on from the board */
            case "erase":
                if ((event.target as HTMLElement).tagName !== "circle") {
                    return;
                }

                let idx: number = Number((event.target as SVGCircleElement).classList[2]);

                circles = circles.slice(0, idx).concat(circles.slice(idx+1, circles.length));

                break;
        }
    
    }

    let points = 6;
    let attachmentCoords: Array<Coordinates> = [];
    
    function calculateRelativeCoords() {
        
        // x = r * cos(θ), y = r * sin(θ)
        for (let i = 0; i < points; i++) {

            let theta: number = i * 2 * Math.PI / points;
            const coords : Coordinates = {
                x: radius * Math.cos(theta - Math.PI/2),
                y: radius * Math.sin(theta - Math.PI/2)
            }
            
            attachmentCoords = attachmentCoords.concat(coords);
        }
    }

    
    // State name which is being dragged
    let currStateName: String = "";
    
    // Relative location of click from the center of the circle
    let clickX: number = 0, clickY: number = 0;         // Variables for cursor
    
    // temporary variable                               // Variables for transition
    let ptRadius: number = 7;
    
    // if pointer is attached to attachment point
    let attached: number = -1;
    
    // line to indicate new transition being dragged out
    let tempTransition: Array<Coordinates> = [];

    // All connected transitions of some state
    // let connectedTx: Array<ConnectedTransition> = [];
    
    // Sidenote, erase might function like a brush later
    // so it will be in this function
    function handleMovement(event: MouseEvent): void {

        switch (currMode) {

            /* When the cursor is moving over the svg tag and the 
               mouse is held down on a circle, it's position is updated */
            case "state":

                if (currStateName === "") {
                    return;
                }

                let mouseX: number = event.clientX -
                    (event.target as SVGCircleElement).parentElement!.getBoundingClientRect().left;
                let mouseY: number = event.clientY -
                    (event.target as SVGCircleElement).parentElement!.getBoundingClientRect().top;                

                // Making transitions move along with states

                let connected = states.get(currStateName)!.connTransitions;

                for (let i = 0; i < connected.length; i++) {
                    if (connected[i].from) {
                        transitions[connected[i].index].bezier[0].x = 
                            mouseX + clickX + attachmentCoords[transitions[connected[i].index].attPt.from].x;
                        transitions[connected[i].index].bezier[0].y = 
                            mouseY + clickY + attachmentCoords[transitions[connected[i].index].attPt.from].y;
                    } else {
                        transitions[connected[i].index].bezier[3].x = 
                            mouseX + clickX + attachmentCoords[transitions[connected[i].index].attPt.to].x;
                        transitions[connected[i].index].bezier[3].y = 
                            mouseY + clickY + attachmentCoords[transitions[connected[i].index].attPt.to].y;
                    }
                }

                states.set(currStateName, {
                    coords: {
                        x: mouseX + clickX,
                        y: mouseY + clickY
                    },
                    connTransitions: connected
                });

                states = states;

                break;
            
            case "transition":

                if (tempTransition.length > 0) {
                    tempTransition[1] = {
                        x: event.clientX - (event.target as HTMLElement).parentElement!.getBoundingClientRect().left,
                        y: event.clientY - (event.target as HTMLElement).parentElement!.getBoundingClientRect().top
                    };
                }
                    
                if ((event.target as HTMLElement).tagName !== "circle") {
                    if ( attached >= 0) {
                        return;
                    }
                    transitionIdx = -1;
                    transitionName = "";
                    return;
                }

                if (points !== attachmentCoords.length) {
                    calculateRelativeCoords();
                }
                
                let id: String = (event.target as HTMLElement).id;

                if ((event.target as HTMLElement).classList.contains("attach-pt") ||
                    (transitionIdx >= 0 && transitionIdx == Number((event.target as SVGCircleElement).id))||
                    (transitionName !== "" && transitionName === (event.target as SVGCircleElement).classList[1])) {
                    return;
                }

                transitionName = (event.target as SVGCircleElement).classList[1];
                transitionIdx = Number(id.slice(6, id.length));

                break;

        }
    }

    
    /* Determines if mouse is clicked and sets appropriate variables according to the mode*/
    function setMovement(event: MouseEvent): void {
        switch (currMode) {
            case "state":
                
                // Here we add a new state
                if ((event.target as HTMLElement).tagName !== "circle") {

                    // Position of the mouse relative to the board
                    const circle: Coordinates = {
                        x: event.clientX - (event.target as HTMLElement).getBoundingClientRect().left,
                        y: event.clientY - (event.target as HTMLElement).getBoundingClientRect().top
                    };

                    // New way to address states, will expand in later commits
                    circles = circles.concat("q"+circles.length);
                    states.set(circles[circles.length-1], {
                        coords: circle,
                        connTransitions: []
                    });

                    return;
                }

                // Here we move an existing state
                currStateName = (event.target as SVGCircleElement).classList[1];

                // Putting circle clicked on to the top
                let idx: number = Number((event.target as SVGCircleElement).classList[2]);

                let tempCircle = circles[idx];
                circles = circles.slice(0, idx).concat(circles.slice(idx + 1, circles.length));
                circles = circles.concat(tempCircle);

                // circle coords - (click coords) = distance of click from center of circle
                clickX = Number((event.target as SVGCircleElement).cx.baseVal.value) - 
                    (event.clientX - (event.target as SVGCircleElement).parentElement!.getBoundingClientRect().left);
                clickY = Number((event.target as SVGCircleElement).cy.baseVal.value) -
                    (event.clientY - (event.target as SVGCircleElement).parentElement!.getBoundingClientRect().top);

                break;
            
            case "transition":

                if (!(event.target as HTMLElement).classList.contains("attach-pt")) {
                    return;
                }
                
                let attID = (event.target as SVGCircleElement).id;
                attachedCircle = (event.target as SVGCircleElement).classList[1];
                let mouseX = event.clientX - (event.target as SVGCircleElement).parentElement!.getBoundingClientRect().left;
                let mouseY = event.clientY - (event.target as SVGCircleElement).parentElement!.getBoundingClientRect().top;

                attached = Number(attID.slice(3, attID.length));
                tempTransition = [
                    {x: Number((event.target as SVGCircleElement).cx.baseVal.value), y: Number((event.target as SVGCircleElement).cy.baseVal.value)},
                    {x: mouseX, y: mouseY}
                ];

                break;
        }
    }

    /* Changes relevant variables when click is released according to the mode*/
    function unsetMovement(event: MouseEvent): void {
        switch (currMode) {
            case "state":
                clickX = clickY = 0;
                currStateName = "";

                break;
            
            case "transition":
                
                if ((event.target as HTMLElement).classList.contains("attach-pt")) {

                    let currCircle = (event.target as SVGCircleElement).classList[1];
                    let attachIdx = (event.target as SVGCircleElement).id;

                    if (currCircle === attachedCircle &&
                        Number(attachIdx.slice(3, attachIdx.length)) === attached) {
                        attachedCircle = "";
                        attached = -1;
                        tempTransition = [];

                        return;
                    }
                    
                    const newTransition : Transition = {
                        bezier: [
                            {x: tempTransition[0].x, y: tempTransition[0].y},
                            {x: 150, y: 150},
                            {x: 150, y: 200},
                            {x: Number((event.target as SVGCircleElement).cx.baseVal.value), y: Number((event.target as SVGCircleElement).cy.baseVal.value)}
                        ],
                        state: {from: attachedCircle, to: currCircle},
                        attPt: {from: attached, to: Number(attachIdx.slice(3, attachIdx.length))}
                    };
                    
                    transitions.push(newTransition);
                    transitions = transitions;

                    let prevState: StateProperties = states.get(attachedCircle)!;
                    prevState.connTransitions.push({
                        index: transitions.length - 1,
                        from: 1
                    });

                    states.set(attachedCircle, {
                        coords: prevState.coords,
                        connTransitions: prevState.connTransitions
                    });

                    prevState = states.get((event.target as SVGCircleElement).classList[1])!;
                    prevState.connTransitions.push({
                        index: transitions.length - 1,
                        from: 0
                    });

                    states.set((event.target as SVGCircleElement).classList[1], {
                        coords: prevState.coords,
                        connTransitions: prevState.connTransitions
                    });

                }
                attachedCircle = "";
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

        {#each circles as circ, index}
            <circle cx={states.get(circ).coords.x} cy={states.get(circ).coords.y} r={radius} />
            <text text-anchor="middle" x={states.get(circ).coords.x} y={states.get(circ).coords.y}>{circ}</text>
            <circle id={"bound-"+index} class={"bounding "+circ+" "+index} cx={states.get(circ).coords.x} cy={states.get(circ).coords.y} r={radius+ptRadius} />
            {#if circ === transitionName} 
                {#each attachmentCoords as coords, idx}
                    <circle class={"attach-pt "+circ} id={"idx"+idx} cx={states.get(circ).coords.x + coords.x}
                            cy={states.get(circ).coords.y + coords.y} r={ptRadius} />
                {/each}
            {/if}
        {/each}

        <!-- Transitions -->
        {#each transitions as t, index}
            <path D={"M "+t.bezier[0].x+","+t.bezier[0].y + " C "+t.bezier[1].x+","+t.bezier[1].y+" "+t.bezier[2].x+","+t.bezier[2].y+" "+t.bezier[3].x+","+t.bezier[3].y}
                  id={"tx-"+index}/>
        {/each}
        {#each transitions as trans}
            {#each trans.bezier as pts}
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