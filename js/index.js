$(document).ready(function () {
    $(".arrow_row").click(function () { 
        $(this)
        .toggleClass("rotate");
        let items = $(this).parent().next();

        if ($(this).prop("class").includes("rotate")) {
            $(items).slideDown(500);
        } else {
            $(items).slideUp(500);

        }

    });

});




