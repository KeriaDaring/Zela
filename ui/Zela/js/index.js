
const {sep, renameDir, renameFile, removeDir, removeFile} = window.__TAURI__.fs;

const isWindows = window.__TAURI__.windows;

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
    // console.log(msg)
    // console.log(msg[0])
    for (let i = 1; i <= 6; i++) {
        let text = "#msg_mini" + i;
        if (i === 6) {
            if (msg[2] === "Folder") {
                $(text).text("无");
                return;
            }
            let num = formatBytes(msg[i - 1]);
            $(text).text(num);
            return;
        }
        $(text).text(msg[i - 1]);
    }
}

function formatBytes(bytes, decimals = 2) {
    if (bytes === 0) return '0 Bytes';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}


$(document).ready(function () {
    $("#search_input").change(async function () {
        let str = document.getElementById("search_input").value;
        if (!str) return;
        // console.log(str.toString())
            

        if (!$("#fun1").hasClass("press")) {
            await access(process_path(str)).then(async r =>
                await add_process()
            );     
            return;
        }
        else {
            invoke("search", {target: str})
            await add_process();
        }
        // if (window.Worker) console.log("yes")


        // myWorker.postMessage(str); // 发送
        // console.log("发送成功");
        // $("#file_pane").html("");

        // myWorker.terminate();
    })
    $("#sidebar").click(async function (e) {
        console.log("fooooold")
        let target = e.target.id
        let id = target.id.split("_").pop();
        $(target)
            .toggleClass("rotate");
        let a = $(target).parent();
        let items = $(a).nextAll();

        if ($(target).prop("class").includes("rotate")) {
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
    // $(".button").click(function () {
    //     $(".press").find().toggleClass("press");
    //     $(this).toggleClass("press");
    //     let num = parseInt($(this).id.split("_").pop());
    //     change(num);
    // })
    // $(".button1").click(function () {
    //     $(".button1 .press").toggleClass("press");
    //     $(this).toggleClass("press");
    // })
    $(".button_block").click(function(e) {
        let target = e.target.closest(".button");
        $(this).find(".press").toggleClass("press");
        $(target).toggleClass("press");
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
    // // })
    //
    $("#file_pane").click(function (e) {
        let target = e.target.closest(".file_title");
        $(".select").toggleClass("select");
         $(target).toggleClass("select");
         let index = target.id;
        if (!index) return;

        update_msg(index);
    })
    $("#file_pane").mousedown(function (e) {
        console.log("鼠标按下去了 " + e.which)
        if (e.which === 3 || e.which === 1) {
            let target = e.target.closest(".file_title");
            $(".select").toggleClass("select");
            $(target).toggleClass("select");

            let index = target.id;

            if (!index) return;

            update_msg(index)
        }
    })

    $("body").click(function (e) {
        let target = e.target.closest(".fold");
        $(".press").toggleClass("press");
        $(target).toggleClass("press");

        let val = $(target).prop("id");
        if (!val) return

        console.log(val)
        let list = val.split("_");
        let id = list[list.length - 1];

        console.log(id);
        invoke("access", {path: process_path(curren_list[id])}).then(async () => {
            await add_process();
        });

    })



    $("#file_pane").mouseup(function (e) {
        console.log("这是你想要的" + e.which);
        if (e.which === 1) {
            let target = e.target.closest(".file_title");
            let des = current_file_msg[target.id - 1]
            let type1 = des[2];
            let id = $(".select").prop("id");
            let msg = current_file_msg[id - 1].slice();
            let type2 = msg[2];

            if (type1 !== "Folder" && type2 !== "Folder") {
                return;
            }

            // invoke("_move", {path1: process_path(msg[1]), path2: process_path(des[1])}).then(async () => {
            //     await refresh();
            // })
            console.log("移动成功")
        }
    })

    $("#file_pane").bind("keypress", async function (e) {
        if (e.keyCode == "13") {
            console.log("回车")
            let val = $('#input').val();
            console.log(val)
            $("#input").closest(".item_name")
                .text(val.slice());
            let span = $("#input").closest(".item_name")
            $(span).text(val);
            let id = $(".select").prop("id");
            let type = current_file_msg[id - 1][2];
            let msg = current_file_msg[id - 1][1];
            // console.log(msg[1] + "   " + val)

            let index = msg.lastIndexOf("/");
            let str = msg.slice(0, index) + "/" + val;

            console.log(str)
            // console.log("here       " + list.split(",").join(sep))
            // let a = des.split(",").pop().push(val);
            // console.log(des + "  final")
            // await invoke("ren ame", {path: process_path(msg[1]), newName: list})
            await renameFile(msg, str);
            await refresh()
        }
    })
    //
    // $("#file_pane").dragLeave(function (e) {
    //     console.log(e.which);
    // })



    $("#file_pane").dblclick(async function (e) {
        let target = e.target.closest(".file_title");
        next_path = [];
        let id = target.id;
        console.log(id);
        let msg = current_file_msg[id - 1].slice();

        //msg
        //name
        //path
        //type
        //time1
        //time2
        //size
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
        // $(".breadcrumb").append(creat_breadcomp(msg[0].slice()));
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

    $("#disk").click(async function (e) {

        $(".fold .press").toggleClass("press");


        let target = e.target.closest(".tile");

        // let classList = target.classList;
        // if (classList.contains("arrow_row")) {
        //
        // }
        if (!target) return;

        $(target).toggleClass("press")
        let id = target.id;
        let index = id.split("_").pop();
        let path = tile_path[index];

        for (let i = 1; i <= 6; i++) {
            let name = "#msg_mini" + i;
            $(name).text("")
        }

        console.log(path)
        console.log(tile_path)
        let path1 = process_path(path.slice());

        // borad_msg = invoke("current_layer_msg", {path: path1});
        await access(path1).then(async () => await add_process())
        next_path = []
        pre_path = [[path]];
        check_control();
        update_msg_current()
    })

    $(".open").click(async function () {
        let index = document.querySelector(".select").id;
        console.log(index)
        let msg = current_file_msg[index - 1];
        console.log(msg)
        let path = msg[1];
        if (msg[2] !== "Folder") {
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
        let id = $(".select").prop("id");
        let msg = current_file_msg[id - 1] ;
        let type = msg[2];
        let arr = msg[1];

        // let index = arr.lastIndexOf("/");
        // let path = arr.slice(0, index) + "/";

        let name = $(".select").find(".item_name").text();

        if (type === "Folder") {
            // await removeDir(name, { dir: new Uint16Array(path.split("/").join().split(""))});

            await invoke("delete_dir", {path:arr.split("/")})
        } else {
            // await removeFile(name, {dir: new Uint16Array(path.split("/").join().split(""))})
            await invoke("delete_file", {path: arr.split("/")});
        }

        await access(pre_path[pre_path.length - 1]).then(async r =>
            await add_process()
        );
    })

    $("#new_file").click(async function (e) {
        let a = pre_path[pre_path.length - 1].slice();
        a.push("新建文件");
        console.log(a)
        await invoke("new_file", {path: a}).then(async () => {
            await refresh()
        });
    })
    $("#new_dir").click(async function (e) {
        let a =  pre_path[pre_path.length - 1].slice();
        a.push("新建文件夹");
        console.log(a)
        await invoke("new_dir", {path: a}).then(async () => {
            await refresh()
        })
        ;
    })

    $("#rename").click(function (e) {
        // let target = e.target.closest(".file_title");
        let elem = document.createElement("input");
        let name = $(".select").find(".item_name").text();
        $(elem)
            .html("<input  value=\"\">")
            .prop("value", name)
            .prop("id", "input");
        console.log("创建成功")
        $(".select")
            .find(".item_name").text("")

        $(".select")
            .find(".item_name").append(elem)
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

async function refresh() {
    await access(pre_path[pre_path.length - 1]).then(async r =>
        await add_process()
    );
    update_msg_current();
    $(".file_title").mouseup(function (e) {
        console.log("这是你想要的" + e.which);
        if (e.which === 1) {
            let target = e.target.closest(".file_title");
            let des = current_file_msg[target.id - 1]
            let id = $(".select").prop("id");
            let msg = current_file_msg[id - 1].slice();
            invoke("move", {path1: process_path(msg[1]), path2: process_path(des[1])}).then(async () => {
                await refresh();
            })
            console.log("移动成功")
        }
    })
}


const invoke = window.__TAURI__.invoke;
// import {appWindow} from "@tauri-apps/api/window";
// import {sep} from "@tauri-apps/api/path"
// const sep = window.__TAURI__.path.separator
// console.log(sep)

function process_path(str) {
    if (!str || typeof str === "Array") return
    // console.log("原始 " + str)
    if (!isWindows) {
        return str.split("\u005C");
    }
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
    console.log(list)
    console.log("____________")
    $(".location>div").text("");
    for (let i of list) {
        // console.log(i)
        console.log("yes")
        $(".location").append(creat_tile(i));
    }
    // let index = 0;
    // $(".tile_name").forEach((n) => {
    //     console.log("初始化名称" + tile_path)
    //     $(n).text(tile_path[index++ + 1].slice())
    // })
}

function creat_tile(msg) {
    // console.log("this  " +msg);
    let tile = document.createElement("div");
    let id = "tile_" + tile_path.length;
    $(tile).html("<div class=\"item press tile\">\n" +
        "                        <img src=\"./imgs/home.svg\" alt=\"\">\n" +
        "                        <div class=\"text tile\"><span class='tile_name'>Home</span></div>\n" +
        "                    </div>")

    let path_arr = process_path(msg);
    // console.log(path_arr)
    tile_path = [...tile_path, msg];
    console.log(tile_path)
    console.log("above")
    $(tile).find(".tile_name").text(msg.slice());
    $(tile).find(".press").prop("id", id);
    return tile;
}

async function access(path) {
    if (!path) {
        console.log("路径非法" + path);
    }
    console.log("这是你要访问的路径" + path)
    await invoke('access', {path: path.slice()});
    let name = path.slice()[path.length - 1].split(".")[0];
    console.log("这是名称" + name);
    check_control();
    // current_file_msg = [];
    $("#file_name").find(".name").text(name)
    $("#name").text(name)
    // borad_msg = await invoke("get_file");
    // update_msg_current()

    update_msg_current()
}

function update_msg_current() {
    for (let i = 1; i <= 6; i++) {
        let text = "#msg_mini" + i;
        // console.log(borad_msg)
        if (borad_msg[i - 1] === "无") {
            $(text).text("无");
            continue;
        }

        $(text).text(borad_msg[i - 1]);
    }
}


async function change(target) {
    if (!target) return;
    invoke("fold", {target});
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
    borad_msg = await invoke("current_layer_msg", {path: pre_path[pre_path.length - 1]});
    update_msg_current();
}


tile_path = [""]

current_file_msg = [];

borad_msg = []

copyposition = []

tree_size = 1;
curren_list = ["", "/Users"];


pre_path = [["/Users"]];
next_path = [];
read_ui();
init();
add_process().then( async () => {
    await init_layers()
});



$(".position .tile").click()


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
        case "PDF" : {
            img = "../imgs/pdf.png";
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
        "                <div class=\"text file\"><span class='item_name'></span></div>\n" +
        "            </div>";
    $(item).find("img").prop("src", img);
    $(item).find(".item").prop("id", id);
    if (img === `../imgs/file.png`) {
        $(item).find("img").css("width", "40px");
    }
    $(item).find(".item_name").text(msg[0] ? msg[0] : "");
    return item;
}

async function add_process() {
    $("#file_pane").html("");
    let times = 0;

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



function layer(msg) {
    let elem = document.createElement("div");
    $(elem).html("<div class=\"item tile mini-title\">\n" +
        "                                <div class=\"left_block\">\n" +
        "                                    <img class='head_img' src=\"./imgs/home.svg\" alt=\"\">\n" +
        "                                    <div class=\"text tile\"><span class='context_name'>Home</span></div>\n" +
        "                                </div>\n" +
        "                                <img  class=\"arrow_row rotate\" src=\"./imgs/Chevron_Down.svg\" alt=\"\">\n" +
        "                            </div>")

    let name = msg[0];
    let img = ""
    switch (msg[2]) {
        case "Folder": {
            img = `../imgs/dir.png`;
            $(elem).find(".head_img").css("width", "30px");
            break;
        }
        case "PDF" : {
            img = "../imgs/pdf.png";
            $(elem).find(".arrow_row").css("display", "none");
            $(elem).find(".head_img").css("width", "20px");
            break;
        }
        default: {
            $(elem).find(".arrow_row").css("display", "none");
            $(elem).find(".head_img").css("width", "20px");
            img = `../imgs/file.png`;
        }
    }
    let id = "context_" + ++tree_size;
    $(elem).find(".context_name").text(name);
    $(elem).find(".head_img").prop("src", img);
    $(elem).prop("id", id).prop("class", "fold tiles");
    return elem;

}
//
async function access_layer(path, current) {
    invoke("access1", {path: process_path(path)})

    let times = 0;
    let record = tree_size;

    while (1) {
        console.log("开始了")
        let list = await invoke("get_file1");
        if (!list) {
            times++;
        } else {
            curren_list.push(list[1].concat());
            console.log(curren_list)
            let hello = layer(list);
            if (!hello) continue;
            let id1 = "#context_" + current;
            $(id1).append(hello);
            console.log("添加成功")
            times = 0;
        }
        if (times > 3000) break;
        console.log(list)


    }
    // console.log("完成一个节点  " + record + "      " +  tree_size)
    if (current === tree_size) return;

    if (current > 100) return
    await access_layer(curren_list[current + 1], current + 1)
}
// [1, 2, 3]
// 3

async function init_layers() {
    await access_layer(curren_list[tree_size], tree_size)
}


