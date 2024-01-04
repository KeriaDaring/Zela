document.onkeydown = function () {
    if (window.event && window.event.keyCode == 123) {
        event.keyCode = 0;
        event.returnValue = false;
    }
    if (window.event && window.event.keyCode == 13) {
        window.event.keyCode = 505;
    }
}

document.oncontextmenu = function (e) {
    // e.preventDefault();

    let target = e.target.closest("div");
    if (!target) return;
    let id = target.id;
    let name = target.classList;
    console.log(name)
    let menu1 = document.querySelector("#contextMenu1");
    let menu2 = document.querySelector("#contextMenu2");
    let menu3 = document.querySelector("#contextMenu3");
    let x = e.clientX - 10 + "px";
    let y = e.clientY - 10 + "px";

    if (name.contains("file")) {
        transform(menu1, x, y);
        return;
    }
    if (name.contains("file_area")) {
        transform(menu2, x, y)
    }
    if (name.contains("tile")) {
        transform(menu3, x, y)
    }

    $(item).css("visibility", "hidden");

    function transform(item, x, y) {
        $(item).css("transform", `translateX(${x}) translateY(${y})`);
        $(item).css("visibility", "visible");
    }
}


function update_msg(index) {
    let msg = current_file_msg[index - 1];
    console.log(msg)
    // console.log(msg[0])
    for (let i = 1; i <= 6; i++) {
        let text = "#msg_mini" + i;
        $(text).text(msg[i - 1]);
    }
}

$(document).ready(function () {
    $("#search_input").change(async function () {
        let str = document.getElementById("search_input").value;
        if (!str) return;
        console.log(str.toString())
        // if (window.Worker) console.log("yes")

        myWorker.postMessage(str); // 发送
        console.log("发送成功");
        $("#file_pane").html("");

        // myWorker.terminate();
    })
    $(".arrow_row").click(async function (e) {
        let target = e.target.closest("img");
        let id = target.id.split("_").pop();
        $(this)
            .toggleClass("rotate");
        let items = $(this).parent().next();

        if ($(this).prop("class").includes("rotate")) {
            $(items).slideDown(200);
        } else {
            $(items).slideUp(200);
        }

        let num = parseInt(id);
        await change(num);
    });
    $(".tag").click(function () {
        $(".current_page").toggleClass("current_page")
        $(this).toggleClass("current_page")
    })
    $(".tag img").click(function () {
        $(this).closest(".tag").toggle("slide:right");
    })
    // $(".fold .item").click(function () {
    //     $(".fold .press").toggleClass("press");
    //     $(this).toggleClass("press")
    // })
    $(".button").click(function () {
        $("#button_area .press").toggleClass("press");
        $(this).toggleClass("press");
        let num = parseInt($(this).id.split("_").pop());
        change(num);
    })
    // $("#file_pane .item").click(function () {
    //     $(".select").toggleClass("select");
    //     $(this).toggleClass("select");
    // })
    //
    // $("#file_pane .item").dblclick(function () {
    //     let id = $(this).id.toString();
    //     let path = current_file_msg[id]
    //     access(path);
    //     add_process();
    // })
    $("#file_pane ").click(function (e) {
        let target = e.target.closest(".file_title");
        $(".select").toggleClass("select");
         $(target).toggleClass("select");
         let index = target.id;
        if (!index) return;

        update_msg(index);
    })
    $("#file_pane").mousedown(function (e) {
        if (e.which === 3) {
            let target = e.target.closest(".file_title");
            $(".select").toggleClass("select");
            $(target).toggleClass("select");

            let index = target.id;
            update_msg(index)
        }
    })

    function bread_change(path) {

    }

    function creat_breadcomp(name) {
        let comp = document.createElement("li");
        $(comp).html("<li class=\"breadcrumb-item\"><a href=\"#\">Home</a></li>");

    }

    $("#file_pane").dblclick(async function (e) {
        let target = e.target.closest(".item");
        next_path = [];
        let id = target.id;
        console.log(id);
        let msg = current_file_msg[id - 1].slice();
        borad_msg = [];
        borad_msg = msg.slice();
        let path = msg[1].slice();
        // console.log(current_file_msg[id][2])
        if (current_file_msg[id - 1][2] !== "Folder") {
            invoke("open", {path: process_path(path)})
            return
        }
        console.log(path)
        pre_path.push(process_path(path.slice()))
        await access(process_path(path)).then(async () => {
            current_file_msg = [];
            await add_process();
        })
        bread_change(pre_path[pre_path - 1]);

    })

    $("#pre").click(async function () {
        await pre()
        check_control();
    })

    $("#next").click(async function () {
        await next()
        check_control();
    })

    $(document).click(function () {
        $(".menu").css("visibility", "hidden")
    })

    $("#sidebar").click(async function (e) {

        $(".fold .press").toggleClass("press");


        let target = e.target.closest(".location .tile");
        if (!target) return;

        $(target).toggleClass("press")
        let id = target.id;
        let index = id.split("_").pop();
        let path = tile_path[index + 1];
        console.log(path)
        let path1 = process_path(path[0]);

        await access(path1).then(async () => await add_process())
        next_path = []
        pre_path = pre_path.slice(0, 1);
        check_control();
    })

    $(".open").click(async function () {
        let index = document.querySelector(".select").id;
        console.log(index)
        let msg = current_file_msg[index - 1];
        console.log(msg)
        let path = msg[1];
        if (msg[2] !== "dir") {
            invoke("open", {path: process_path(path)})
            return
        }
        await access(process_path(path)).then(async r => await add_process())
        pre_path.push(process_path(path.slice()))
        check_control()
    })


    $(".copy").click(function() {
        let index = document.querySelector(".select").id;
        copyposition = current_file_msg[index - 1];
    })

    $("#paste").click(async function () {
            let path = [...process_path(borad_msg[1]), copyposition[0]]
            console.log(path)
            console.log(copyposition)
            //todo!!按钮亮暗
            await invoke("copy", {path1: process_path(copyposition[1]), path2: path})
            await access(process_path(borad_msg.slice()[1])).then(async r =>
                await add_process()
            );
            copyposition = []
        }
    );
    $(".delete").click(async function () {
        let index = document.querySelector(".select").id;
        let path = process_path(current_file_msg[index - 1].slice()[1]);
        let type = path[2].slice();
        console.log(path);
        if (type === "Folder")
            invoke("delete_dir_all", {path:path})
        else {
            invoke("delete_file", {path: path});
        }
        await access(process_path(borad_msg.slice()[1])).then(async r =>
            await add_process()
        );
    })

    $("#rename").click(function (e) {
        let target = e.target.closest(".file");
    })
});

function check_control() {
    if (pre_path.length > 1) {
        $("#pre").css("color", "white");
    } else {
        $("#pre").css("color", "grey")
    }
    if (next_path.length > 0) {
        $("#next").css("color", "white");
    } else {
        $("#next").css("color", "grey");
    }
}


const invoke = window.__TAURI__.invoke;
// import {appWindow} from "@tauri-apps/api/window";
// import {sep} from "@tauri-apps/api/path"
// const sep = window.__TAURI__.path.separator
// console.log(sep)

function process_path(str) {
    return str.split("/").join().split("\\").join().split(",");
}

async function test() {
    let a = await invoke('test');
    for (let i of a) {
        console.log(i);
    }
}

async function read_ui() {
    invoke("read_ui").then((result) => {
        for (let i = 0; i < 4; i++) {
            if ((Number)(result[i]) === 1) {
                let name = "#button_" + (i + 1);
                if (i === 0 || i === 1) {
                    $(name).click();
                    return;
                }
                $(name).addClass("press")
            }
        }
    })
    console.log("初始化tile")
    let list = await init_tiles();
    $(".location>div").text("");
    for (let i of list) {
        // console.log(i)
        $(".location").append(creat_tile(i));
    }
}

function creat_tile(msg) {
    // console.log(msg);
    let tile = document.createElement("div");
    let id = "tile_" + tile_path.length;
    $(tile).html("<div class=\"item press tile\">\n" +
        "                        <img src=\"./imgs/home.svg\" alt=\"\">\n" +
        "                        <div class=\"text tile\"><span>Home</span></div>\n" +
        "                    </div>")

    let path_arr = process_path(msg);
    // console.log(path_arr)
    tile_path = [tile_path, [msg]];
    console.log(tile_path)
    console.log(tile_path)
    $(tile).find(".text").text(path_arr[path_arr.length - 1]);
    $(tile).id = id;
    return tile;
}

async function access(path) {
    await invoke('access', {path: path});
    let name = path.slice()[path.length - 1].split(".")[0];
    check_control();
    // current_file_msg = [];
    $("#name").text(name.split(".")[0])


    update_msg_current()
}
function update_msg_current() {
    for (let i = 1; i <= 6; i++) {
        let text = "#msg_mini" + i;
        $(text).text(borad_msg[i - 1]);
    }
}


async function change(target) {
    if (!target) return;
    await invoke("fold", {target});
}

async function pre() {
    if (pre_path.length === 1) return;
    console.log(pre_path)
    next_path.push(pre_path.pop());
    console.log(next_path);
    current_file_msg = []
    borad_msg = [];
    borad_msg = pre_path[pre_path.length - 1].slice();
    await access(pre_path[pre_path.length - 1]).then(async () => {
        await add_process()
    })
}

async function next() {
    if (next_path.length === 0) return;
    console.log(next_path)
    pre_path.push(next_path.pop());
    console.log(pre_path)
    console.log(next_path);
    current_file_msg = []
    borad_msg = [];
    borad_msg = pre_path[pre_path.length - 1].slice();
    console.log(borad_msg)
    await access(pre_path[pre_path.length - 1]).then(async () => {
        await add_process();
    });
}

async function init() {
    await access(pre_path[0])
}


tile_path = [""]

current_file_msg = [];

borad_msg = []

copyposition = []


pre_path = [["Users"]];
next_path = [];
read_ui();
init();
add_process();


myWorker = new Worker('find.js');
myWorker.addEventListener("message", (event) => {
    let msg = event.data;
    console.log("接收成功");
    if (!msg) {
        return;
    }
    console.log(msg)
    let hello = item(msg);
    $("#file_pane").append(hello);
})

function push(path, name) {
    return path + sep + name;
}


function item(msg) {
    if (!msg) return
    current_file_msg.push(msg);
    // console.log(current_file_msg);
    let img = "";
    switch (msg[2]) {
        case "Folder": {
            img = `../imgs/dir.png`;
            break;
        }
        default: {
            img = `../imgs/file.png`;
        }
    }

    let id = current_file_msg.length;
    // console.log(id)
    let item = document.createElement("div");
    item.innerHTML = "<div class=\"item file file_title\">\n" +
        "                <div class='file'><img alt=\"\" src=\"./imgs/dir.png\"></div>\n" +
        "                <div class=\"text file\"><span></span></div>\n" +
        "            </div>";
    $(item).find("img").prop("src", img);
    $(item).find(".item").prop("id", id);
    if (img === `../imgs/file.png`) {
        $(item).find("img").css("width", "40px");
    }
    $(item).find(".text span").text(msg[0] ? msg[0] : "");
    return item;
}

async function add_process() {
    $("#file_pane").html("");
    let times = 0;
    borad_msg = await invoke("get_file");
    update_msg_current(borad_msg)
    while (1) {
        let list = await invoke("get_file");
        if (!list) times++;
        if (times > 1000) break;
        // console.log(list)
        let hello = item(list);
        if (!hello) continue;
        $("#file_pane").append(hello);
        times = 0;

    }
    // $("#file_pane").removeChild($("#1"))
}

async function add_tiles(path) {
    await invoke("add_tiles", {path: path});
}

async function init_tiles() {
    return await invoke("init_tiles");
}

async function remove_tiles(target) {

}

