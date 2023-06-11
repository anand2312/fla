<script>
    let currMode = "cursor";
    let prev = "placeholder";
    function mode(event) {
        console.log(prev);
        currMode = event.target.id;
        event.target.classList.add("clicked");
        if (typeof(prev) === 'object') {
            console.log("yes");
            prev.classList.remove("clicked");
        } else {
            event.target.parentElement.firstChild.classList.remove("clicked");
        }
        prev = event.target;
    }
</script>

<div class="board">
    <div class="modes">
        <div id="cursor" class="clicked" on:click={mode}>cursor</div>
        <div id="state" on:click={mode}>state</div>
        <div id="transition" on:click={mode}>transition</div>
        <div id="erase" on:click={mode}>erase</div>
    </div>
</div>

current mode is: {currMode}
 

<style>
    .board {
        width: 100%;
        background-color:  #274c43;
        border: black solid 5px;
        border-radius: 1em;
        color: aliceblue;
    }

    .modes {
        border: white dotted 1px;
        width: 5em;
        color: beige;
    }

    .modes > div {
        padding: 1em;
    }

    .clicked {
        background-color: rgba(0, 0, 0, 0.4);
    }
</style>