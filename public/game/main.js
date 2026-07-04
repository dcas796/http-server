/** @type {HTMLCanvasElement | null} */
let canvas = null
/** @type {CanvasRenderingContext2D | null} */
let ctx = null
/** @type {[number, number] | null} */
let lastMousePos = null

function resizeCanvasToWindow() {
    if (canvas === null) canvas = document.querySelector("canvas#main-canvas")

    canvas.width = window.innerWidth
    canvas.height = window.innerHeight
    changeColor()
}

function resetLastMousePos() {
    console.log("Mouse leave")
    lastMousePos = null
}

/** @type {(ev: MouseEvent) => any} */
function drawOnMouse(event) {
    if (canvas === null) canvas = document.querySelector("canvas#main-canvas")
    if (ctx === null) ctx = canvas.getContext("2d")

    if (lastMousePos !== null) {
        ctx.beginPath()
        ctx.moveTo(lastMousePos[0], lastMousePos[1])
        ctx.lineTo(event.clientX, event.clientY)
        ctx.stroke()
        ctx.closePath()
    }

    lastMousePos = [event.clientX, event.clientY]
}

function clearCanvas() {
    if (canvas === null) canvas = document.querySelector("canvas#main-canvas")
    if (ctx === null) ctx = canvas.getContext("2d")

    ctx.clearRect(0, 0, canvas.width, canvas.height)
}

const colors = ["black", "silver", "gray", "white", "maroon", "red", "purple", "fuchsia", "green", "lime", "olive", "yellow", "navy", "blue", "teal", "aqua", "aliceblue", "antiquewhite", "aqua", "aquamarine", "azure", "beige", "bisque", "black", "blanchedalmond", "blue", "blueviolet", "brown", "burlywood", "cadetblue", "chartreuse", "chocolate", "coral", "cornflowerblue", "cornsilk", "crimson", "darkblue", "darkcyan", "darkgoldenrod", "darkgray", "darkgreen", "darkgrey", "darkkhaki", "darkmagenta", "darkolivegreen", "darkorange", "darkorchid", "darkred", "darksalmon", "darkseagreen", "darkslateblue", "darkslategray", "darkslategrey", "darkturquoise", "darkviolet", "deeppink", "deepskyblue", "dimgray", "dimgrey", "dodgerblue", "firebrick", "floralwhite", "forestgreen", "fuchsia", "gainsboro", "ghostwhite", "gold", "goldenrod", "gray", "green", "greenyellow", "honeydew", "hotpink", "indianred", "indigo", "ivory", "khaki", "lavender", "lavenderblush", "lawngreen", "lemonchiffon", "lightblue", "lightcoral", "lightcyan", "lightgoldenrodyellow", "lightgray", "lightgreen", "lightgrey", "lightpink", "lightsalmon", "lightseagreen", "lightskyblue", "lightslategray", "lightslategrey", "lightsteelblue", "lightyellow", "lime", "limegreen", "linen", "maroon", "mediumaquamarine", "mediumblue", "mediumorchid", "mediumpurple", "mediumseagreen", "mediumslateblue", "mediumspringgreen", "mediumturquoise", "mediumvioletred", "midnightblue", "mintcream", "mistyrose", "moccasin", "navajowhite", "navy", "oldlace", "olive", "olivedrab", "orange", "orangered", "orchid", "palegoldenrod", "palegreen", "paleturquoise", "palevioletred", "papayawhip", "peachpuff", "peru", "pink", "plum", "powderblue", "purple", "rebeccapurple", "red", "rosybrown", "royalblue", "saddlebrown", "salmon", "sandybrown", "seagreen", "seashell", "sienna", "silver", "skyblue", "slateblue", "slategray", "slategrey", "snow", "springgreen", "steelblue", "tan", "teal", "thistle", "tomato", "turquoise", "violet", "wheat", "white", "whitesmoke", "yellow", "yellowgreen"]
function changeColor() {
    if (canvas === null) canvas = document.querySelector("canvas#main-canvas")
    if (ctx === null) ctx = canvas.getContext("2d")

    ctx.strokeStyle = colors[Math.floor(Math.random() * (colors.length - 1))]
}

/** @type {(ev: KeyboardEvent) => any} */
function keydown(event) {
    switch (event.key) {
        case " ":
            clearCanvas()
            break
        case "c":
            changeColor()
            break
        default:
            break
    }
}

document.body.addEventListener("mouseleave", resetLastMousePos)
window.addEventListener("mousemove", drawOnMouse)
window.addEventListener("keydown", keydown)
window.addEventListener("load", resizeCanvasToWindow)
window.addEventListener("resize", resizeCanvasToWindow)
