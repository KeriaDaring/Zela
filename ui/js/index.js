

$(document).ready(function () {
    $(".arrow_row").click(function () {
        $(this)
        .toggleClass("rotate");
        let items = $(this).parent().next();

        if ($(this).prop("class").includes("rotate")) {
            $(items).slideDown(200);
        } else {
            $(items).slideUp(200);
        }
        let id = $(this).id.toString().split("_").pop();

        let num = parseInt(id);
        change(num);
    });
    $(".tag").click(function () {
        $(".current_page").toggleClass("current_page")
        $(this).toggleClass("current_page")
    })
    $(".tag img").click(function () {
        $(this).closest(".tag").toggle("slide:right");
    })
    $(".fold .item").click(function () {
        $(".fold .press").toggleClass("press");
        $(this).toggleClass("press")
    })
    $(".button").click(function () {
        $("#button_area .press").toggleClass("press");
        $(this).toggleClass("press");
        let num = parseInt($(this).id.split("_").pop());
        change(num);
    })
    $("#file_pane .item").click(function () {
        $(".select").toggleClass("select");
        $(this).toggleClass("select");
    })

    $("#file_pane .item").dblclick( function () {

    })

});


const invoke = window.__TAURI__.invoke;


async function test() {
    let a = await invoke('test');
    for (let i of a) {
        console.log(i);
    }
}

async function read_ui() {
    let list = [];
    invoke("read_ui").then((result) => {

        for (let i = 0; i < 4 ; i++) {
            if ((Number)(result[i]) === 1) {
                let name = "#button_" + i;
                if (i === 0 || i === 1) {
                    $(name).click();
                    return;
                }
                $(name).addClass("press")
            }
        }
    })
    return list;
}

async function access(path) {
    await invoke('access', {path});
    let list = await invoke('get_file');
    for (let i of list) {
        console.log(i);
    }
}

async function change(target) {
    invoke("fold", {target});
}




let tilepath = []

let current_file_msg = [];


let current_path = [];
read_ui();
access("/Users/keria/Desktop");

function push(path, name) {
    return path + sep + name;
}
const sep = "/";

function item(msg) {
    current_file_msg.push(msg);
    let img = "";
    switch (msg[2]) {
        case "dir": {
            img = `..${sep}..${sep}imgs${sep}dir.png`;
        }
        default: {
            img = `..${sep}..${sep}imgs${sep}file.png`;
        }
    }
    return "<div class=\"item\">\n" +
        "                <div><img alt=\"\" src=\"${img}\"></div>\n" +
        "                <div class=\"text\"><span>Usaasdasda</span></div>\n" +
        "            </div>"
}

function add_process() {

}



