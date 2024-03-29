<script lang="ts">

    interface Coordinates {
        x: number,
        y: number
    };

    interface Connection<T> {
        from: T,
        to: T
    };

    interface ConnectionProperties {
        name: String,
        angle: number
    };

    interface Transition {
        bezier: Array<Coordinates>,
        state: Connection<ConnectionProperties>,
        attPt: Connection<number>
    };


    interface ConnectedTransition {
        name: String,
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
    
    let transitionName: String = "";
    
    // Temporary name, renaming later
    let paths: Array<String> = [];
    // Path name => Transition details (bezier coords, to and from state/attachment pt)
    let transitions = new Map<String, Transition>();

    // Gives new state number
    let stateMax: number = 0;
    // Gives new transition number
    let transitionMax: number = 0;

    let attachedCircle: String = "";
    
    // shortcuts mapping
    shortcutMap.set("1", modes[0]);
    shortcutMap.set("2", modes[1]);
    shortcutMap.set("3", modes[2]);

    function shortcuts(event: KeyboardEvent): void {

        if (event.key !== "2" && Array.from(shortcutMap.keys()).includes(event.key)) {
            transitionName = "";
        }
        if (Array.from(shortcutMap.keys()).includes(event.key)) {
            currMode = shortcutMap.get(event.key)!;
        }
    }

    // Array to store names of states in order of rendering
    let circles: Array<String> = [];

    // State Name => State Properties (coorinates and connected transitions)
    let states = new Map<String, StateProperties>();
    
    // Radius of each state in pixels
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
            
            
            /* Removing (for now, only) states clicked on and connected transitions from the board */
            case "erase":

                if ((event.target as HTMLElement).tagName === "circle") {
                    // For erasing transitions on deleting states, the transition name has to be removed from the
                    // path array using linear search, and connected states' connected transitions are to be updated,
                    // again using linear search, which is making me uncomfortable

                    let idx: number = Number((event.target as SVGCircleElement).classList[2]);
                    
                    let stateTransitions: Array<ConnectedTransition> = states.get(circles[idx])!.connTransitions;
                    
                    for (let i = 0; i < stateTransitions.length; i++) {

                        if (i > 0 && stateTransitions[i].name === stateTransitions[i-1].name) {
                            continue;
                        }

                        // Removing records of transition to be deleted from connected state other than the one being deleted
                        let trx: Transition = transitions.get(stateTransitions[i].name)!;
                        if (stateTransitions[i].from) {
                            let connTrx: Array<ConnectedTransition> = states.get(trx.state.to.name)!.connTransitions;
                            for (let j = 0; j < connTrx.length; j++) {
                                if (connTrx[j].name === stateTransitions[i].name) {
                                    connTrx = connTrx.slice(0, j).concat(connTrx.slice(j+1, connTrx.length));
                                    j--;
                                }
                            }
                            states.set(trx.state.to.name, {
                                coords: states.get(trx.state.to.name)!.coords,
                                connTransitions: connTrx
                            });
                        } else {
                            let connTrx: Array<ConnectedTransition> = states.get(trx.state.from.name)!.connTransitions;
                            for (let j = 0; j < connTrx.length; j++) {
                                if (connTrx[j].name === stateTransitions[i].name) {
                                    connTrx = connTrx.slice(0, j).concat(connTrx.slice(j+1, connTrx.length));
                                    j--;
                                }
                            }
                            states.set(trx.state.from.name, {
                                coords: states.get(trx.state.from.name)!.coords,
                                connTransitions: connTrx
                            });
                        }

                        // removing transitions from the path array and the hash map
                        transitions.delete(stateTransitions[i].name);

                        for (let j = 0; j < paths.length; j++) {
                            if (paths[j] === stateTransitions[i].name) {
                                paths = paths.slice(0, j).concat(paths.slice(j+1, paths.length));
                                break;
                            }
                        }
                    }

                    // Finally deleting the state clicked on                
                    states.delete(circles[idx]);
                    circles = circles.slice(0, idx).concat(circles.slice(idx+1, circles.length));

                } else if ((event.target as HTMLElement).tagName === "path") {
                    
                    // remove associated state entries as connected states
                    let trxName: String = (event.target as SVGPathElement).id;
                    let trx: Connection<ConnectionProperties> = transitions.get(trxName)!.state;

                    let connectedStateTrx: Array<ConnectedTransition> = 
                        states.get(trx.to.name)!.connTransitions;

                    for (let i = 0; i < connectedStateTrx.length; i++) {
                        if (connectedStateTrx[i].name === trxName) {
                            connectedStateTrx = connectedStateTrx.slice(0, i).concat(
                                connectedStateTrx.slice(i+1, connectedStateTrx.length)
                            );
                            i--;
                        }
                    }
                    
                    states.set(trx.to.name, {
                        coords: states.get(trx.to.name)!.coords,
                        connTransitions: connectedStateTrx
                    });

                    if (trx.to.name !== trx.from.name) {

                        connectedStateTrx = states.get(trx.from.name)!.connTransitions;

                        for (let i = 0; i < connectedStateTrx.length; i++) {
                            if (connectedStateTrx[i].name === trxName) {
                                connectedStateTrx = connectedStateTrx.slice(0, i).concat(
                                    connectedStateTrx.slice(i+1, connectedStateTrx.length)
                                );
                                i--;
                            }
                        }
                        
                        states.set(trx.from.name, {
                            coords: states.get(trx.from.name)!.coords,
                            connTransitions: connectedStateTrx
                        });

                    }

                    // Now removing the transition from paths array and the hash map
                    transitions.delete(trxName);

                    for (let i = 0; i < paths.length; i++) {
                        if (paths[i] === trxName) {
                            paths = paths.slice(0, i).concat(paths.slice(i+1, paths.length));
                            break;
                        }
                    }
                }

                break;
        }
    
    }

    let points = 8;
    let attachmentCoords: Array<Coordinates> = [];
    
    function calculateRelativeCoords() {
        
        // x = r * cos(θ), y = r * sin(θ)
        for (let i = 0; i < points; i++) {

            let theta: number = i * 2 * Math.PI / points;
            const coords : Coordinates = {
                x: radius * Math.cos(theta),
                y: radius * Math.sin(theta)
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
    
    // if mouse pointer is attached to attachment point
    let attached: number = -1;
    
    // line to indicate new transition being dragged out
    let tempTransition: Array<Coordinates> = [];
    
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
                let stateCoords: Coordinates = states.get(currStateName)!.coords;
                let connected: Array<ConnectedTransition> = states.get(currStateName)!.connTransitions;

                for (let i = 0; i < connected.length; i++) {
                    if (connected[i].from) {

                        // Coords being set for attachment points
                        transitions.get(connected[i].name)!.bezier[0].x = mouseX + clickX +
                            attachmentCoords[transitions.get(connected[i].name)!.attPt.from].x;
                        transitions.get(connected[i].name)!.bezier[0].y = mouseY + clickY +
                            attachmentCoords[transitions.get(connected[i].name)!.attPt.from].y;

                        // Coords being set for handles
                        let secondCircleCoords: Coordinates = states.get(
                            transitions.get(connected[i].name)!.state.to.name
                        )!.coords;

                        let dist: number;

                        if (transitions.get(connected[i].name)!.state.from.name !== 
                            transitions.get(connected[i].name)!.state.to.name ) {
                            dist = Math.sqrt(
                                Math.pow(stateCoords.x - secondCircleCoords.x, 2) +
                                Math.pow(stateCoords.y - secondCircleCoords.y, 2)
                            ) / 7;
                        } else {
                            dist = 100;
                        }

                        // Adjusting moving state's handle
                        let theta1: number = transitions.get(connected[i].name)!.state.from.angle;
                        transitions.get(connected[i].name)!.bezier[1].x = mouseX + clickX + 
                            (radius + dist) * Math.cos(theta1);
                        transitions.get(connected[i].name)!.bezier[1].y = mouseY + clickY + 
                            (radius + dist) * Math.sin(theta1) * -1;

                        // Adjusting connected state's handle
                        let theta2: number = transitions.get(connected[i].name)!.state.to.angle;
                        transitions.get(connected[i].name)!.bezier[2].x = secondCircleCoords.x +
                            (radius + dist) * Math.cos(theta2);
                        transitions.get(connected[i].name)!.bezier[2].y = secondCircleCoords.y +
                            (radius + dist) * Math.sin(theta2) * -1;

                    } else {

                        // Coords being set for attachment points
                        transitions.get(connected[i].name)!.bezier[3].x = mouseX + clickX +
                            attachmentCoords[transitions.get(connected[i].name)!.attPt.to].x;
                        transitions.get(connected[i].name)!.bezier[3].y = mouseY + clickY +
                            attachmentCoords[transitions.get(connected[i].name)!.attPt.to].y;

                        // Coords being set for handles
                        let secondCircleCoords: Coordinates = states.get(
                            transitions.get(connected[i].name)!.state.from.name
                        )!.coords;

                        let dist: number = Math.sqrt(
                            Math.pow(stateCoords.x - secondCircleCoords.x, 2) +
                            Math.pow(stateCoords.y - secondCircleCoords.y, 2)
                        ) / 7;

                        if (transitions.get(connected[i].name)!.state.from.name !== 
                            transitions.get(connected[i].name)!.state.to.name ) {
                            dist = Math.sqrt(
                                Math.pow(stateCoords.x - secondCircleCoords.x, 2) +
                                Math.pow(stateCoords.y - secondCircleCoords.y, 2)
                            ) / 7;
                        } else {
                            dist = 100;
                        }

                        // Adjusting moving state's handle
                        let theta1: number = transitions.get(connected[i].name)!.state.to.angle;
                        transitions.get(connected[i].name)!.bezier[2].x = mouseX + clickX + 
                            (radius + dist) * Math.cos(theta1);
                        transitions.get(connected[i].name)!.bezier[2].y = mouseY + clickY + 
                            (radius + dist) * Math.sin(theta1) * -1;

                        // Adjusting connected state's handle
                        let theta2: number = transitions.get(connected[i].name)!.state.from.angle;
                        transitions.get(connected[i].name)!.bezier[1].x = secondCircleCoords.x +
                            (radius + dist) * Math.cos(theta2);
                        transitions.get(connected[i].name)!.bezier[1].y = secondCircleCoords.y +
                            (radius + dist) * Math.sin(theta2) * -1;

                    }
                }

                transitions = transitions;

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
                    transitionName = "";
                    return;
                }

                if (points !== attachmentCoords.length) {
                    calculateRelativeCoords();
                }
                
                if ((event.target as HTMLElement).classList.contains("attach-pt") ||
                    (transitionName !== "" && transitionName === (event.target as SVGCircleElement).classList[1])) {
                    return;
                }

                transitionName = (event.target as SVGCircleElement).classList[1];
                break;

        }
    }

    
    /* Determines if mouse is clicked and sets appropriate variables according to the mode*/
    function setMovement(event: MouseEvent): void {
        switch (currMode) {
            case "state":
                
                // Here we add a new state
                if ((event.target as HTMLElement).tagName !== "circle") {

                    if ((event.target as HTMLElement).classList.contains("transition")) {
                        return;
                    }

                    // Position of the mouse relative to the board
                    const circle: Coordinates = {
                        x: event.clientX - (event.target as HTMLElement).getBoundingClientRect().left,
                        y: event.clientY - (event.target as HTMLElement).getBoundingClientRect().top
                    };

                    // New way to address states, will expand in later commits
                    let newState: String = "q" + stateMax;

                    circles = circles.concat(newState);
                    states.set(newState, {
                        coords: circle,
                        connTransitions: []
                    });

                    stateMax++;

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
                    {
                        x: Number((event.target as SVGCircleElement).cx.baseVal.value),
                        y: Number((event.target as SVGCircleElement).cy.baseVal.value)
                    },
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

                    // all handle points must go radially outwards, so the polar coords system is appropriate

                    // angle of first cirle's attachment point
                    let theta1: number = Math.atan((states.get(attachedCircle)!.coords.y - tempTransition[0].y) /
                                                   (tempTransition[0].x - states.get(attachedCircle)!.coords.x));

                    if (tempTransition[0].x - states.get(attachedCircle)!.coords.x < 0) {
                        theta1 += Math.PI;
                    } 

                    // angle of second cirle's attachment point
                    let theta2: number = Math.atan(
                        (states.get(currCircle)!.coords.y - Number((event.target as SVGCircleElement).cy.baseVal.value)) / 
                        (Number((event.target as SVGCircleElement).cx.baseVal.value) - states.get(currCircle)!.coords.x)
                    );

                    if (Number((event.target as SVGCircleElement).cx.baseVal.value) - states.get(currCircle)!.coords.x < 0) {
                        theta2 += Math.PI;
                    }

                    // r is just circle radius, defined earlier
                    let handleLen: number;
                    if (states.get(attachedCircle)! !== states.get(currCircle)!) {
                        handleLen = Math.sqrt(
                            Math.pow(states.get(attachedCircle)!.coords.x - states.get(currCircle)!.coords.x, 2) +
                            Math.pow(states.get(attachedCircle)!.coords.x - states.get(currCircle)!.coords.x, 2)
                        ) / 7;
                    } else {
                        handleLen = 100;
                    }

                    let handles: Array<Coordinates> = [
                        //circle 1:
                        {
                            x: (radius + handleLen) * Math.cos(theta1) + states.get(attachedCircle)!.coords.x,
                            y: (radius + handleLen) * Math.sin(theta1) * -1 + states.get(attachedCircle)!.coords.y
                        },
                        //circle 2:
                        {
                            x: (radius + handleLen) * Math.cos(theta2) + states.get(currCircle)!.coords.x,
                            y: (radius + handleLen) * Math.sin(theta2) * -1 + states.get(currCircle)!.coords.y
                        }
                    ];
                    
                    for (let i = 0; i < handles.length; i++) {
                        handles[i].x = handles[i].x >= 0 ? handles[i].x : 0;
                        handles[i].y = handles[i].y >= 0 ? handles[i].y : 0;
                    }
                    
                    const newTransition: Transition = {
                        bezier: [
                            {x: tempTransition[0].x, y: tempTransition[0].y},
                            handles[0],
                            handles[1],
                            {x: Number((event.target as SVGCircleElement).cx.baseVal.value),
                             y: Number((event.target as SVGCircleElement).cy.baseVal.value)}
                        ],
                        state: {
                            from: {
                                name: attachedCircle,
                                angle: theta1
                            },
                            to: {
                                name: currCircle,
                                angle: theta2
                            }
                        },
                        attPt: {from: attached, to: Number(attachIdx.slice(3, attachIdx.length))}
                    };

                    let newTransitionName = "t" + transitionMax;
                    
                    paths = paths.concat(newTransitionName);
                    transitions.set(newTransitionName, newTransition);

                    let prevState: StateProperties = states.get(attachedCircle)!;
                    prevState.connTransitions.push({
                        name: newTransitionName,
                        from: 1
                    });

                    states.set(attachedCircle, {
                        coords: prevState.coords,
                        connTransitions: prevState.connTransitions
                    });

                    prevState = states.get((event.target as SVGCircleElement).classList[1])!;
                    prevState.connTransitions.push({
                        name: newTransitionName,
                        from: 0
                    });

                    states.set((event.target as SVGCircleElement).classList[1], {
                        coords: prevState.coords,
                        connTransitions: prevState.connTransitions
                    });

                    transitionMax++;
                }
                attachedCircle = "";
                attached = -1;
                tempTransition = [];

                break;
        }
    }

    let arrowSize: number = 15; // pixels

    function getArrowCoords(theta: number): Array<Coordinates> {
        // origin is on the point of the arrow
        /*
            y  0
              / 
             /
            <--   x
             \
              \ assume this is the arrow, 45 deg internal angles
               1
            at arrow point, set origin and rotate through theta
            x = y, so diagonal length = sqrt(2) * arrowSize
            rotate as radius, convert to coordinate system

        */

        if (theta % (2*Math.PI) === 0) {
            return [
                {x: arrowSize, y: -arrowSize},
                {x: arrowSize, y:  arrowSize}
            ];
        }
        
        let lineLen: number = Math.SQRT2 * arrowSize;

        let retVal: Array<Coordinates> = [
            {
                x: lineLen * Math.cos(theta + Math.PI/4),
                y: -lineLen * Math.sin(theta + Math.PI/4)
            },
            {
                x: lineLen * Math.cos(theta - Math.PI/4),
                y: -lineLen * Math.sin(theta - Math.PI/4)

            }
        ];

        return retVal;

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
            {@const iterState = states.get(circ)?.coords}
            <circle cx={iterState.x} cy={iterState.y} r={radius} />
            <text text-anchor="middle" x={iterState.x} y={iterState.y}>{circ}</text>
            <circle id={"bound-"+index} class={"bounding "+circ+" "+index} cx={iterState.x} cy={iterState.y} r={radius+ptRadius} />
            {#if circ === transitionName} 
                {#each attachmentCoords as coords, idx}
                    <circle class={"attach-pt "+circ} id={"idx"+idx} cx={iterState.x + coords.x}
                            cy={iterState.y + coords.y} r={ptRadius} />
                {/each}
            {/if}
        {/each}

        {#each paths as p, index}
            // Transition
            {@const pts = transitions.get(p).bezier}
            <path D={"M "+pts[0].x+","+pts[0].y + " C "+pts[1].x+","+pts[1].y+" "+pts[2].x+","+pts[2].y+" "+pts[3].x+","+pts[3].y}
                  id={"t"+index} class="transition" />
            // Arrow
            {@const arrowXY = getArrowCoords(transitions.get(p)?.state.to.angle)}
            <path D="M {pts[3].x + arrowXY[0].x},{pts[3].y + arrowXY[0].y} {pts[3].x},{pts[3].y} {pts[3].x + arrowXY[1].x},{pts[3].y + arrowXY[1].y}" />
        {/each}

        {#each paths as p}
            <!-- I do not like this workaround -->
            {@const iterTransition = transitions.get(p)?.bezier}
            {#each iterTransition as pts}
                <circle cx={pts.x} cy={pts.y} r={4} />
            {/each}
        {/each}

    </svg>
</div>

<div>
    <svg>
        <path D="M 140,140 100,100 140,140 100,180" id="arrow"/>
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

    #arrow {
        /* transform: rotateZ(20deg); */
    }

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