$(".arrow_row").click(async function (e) {
    console.log("fooooold")
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


