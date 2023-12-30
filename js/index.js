

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

    });
    $(".tag").click(function () {
        $(".current_page").toggleClass("current_page")
        $(this).toggleClass("current_page")
    })
    $(".tag img").click(function () {
        $(this).closest(".tag").toggle("slide:right");
    })
});

// import { invoke } from "@tauri-apps/api/tauri";
const invoke = window.__TAURI__.invoke;

async function test() {
    let a = await invoke('test');
}

test()


async function initialUi() {
    let a = await invoke('read_ui');

    for (let i of a ) {
        console.log(i);
    }
}

initialUi()



