// diaries.html 页面脚本
async function get_diaries() {
    let year = new Date().getFullYear();
    let month = new Date().getMonth() + 1;
    let day = new Date().getDate();
    let response = await fetch('/diary/list/' + year + '/' + month + '/' + day);
    diaries = [];
    if (response.ok) {
        this.diaries = await response.json();
        // 解析出数据
        /*
        data.forEach((diary, index) => {
            console.log(`Diary ${index + 1}`);
            console.log(`- Name: ${diary.diary_name}`);
            console.log(`- Path: ${diary.diary_path}`);
        });
        */
       return diaries;
    } else {
        alert("HTTP-Error: " + response.status);
    }
}   