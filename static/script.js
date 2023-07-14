let selectForm = document.getElementById('select-form');
let uploadForm = document.getElementById('upload-form');
let fileInput = document.getElementById('file-input');

let tbody = document.querySelector(".tbody");

function init() {
    
    selectForm.addEventListener("submit", (e) => {
        e.preventDefault();
        const file_path = selectForm.querySelector("#file_path").value;
        fetch("/file_list", {
            method: "POST",
            body: new URLSearchParams({file_path})
        })
        .then((response) => response.json())
        .then(folder => {
            console.log(folder);
            console.log("folder_name: ", folder.folder_name);
            console.log("folder_files: ", folder.folder_files);
            const files = folder.folder_files;
            var html = ""
            files.forEach(file => {
                html += "<tr>"+"<td>"+file.file_name.toString()+"</td>"+"<td>"+file.file_path.toString()+"</td>"+"<td><a href= "+ file.link_path.toString() +">"+ file.file_name.toString() +"</a></td>"+"</tr>";
            })
            tbody.innerHTML = html;
        })
    });

    uploadForm.addEventListener("submit", (e) => {
        e.preventDefault();
        fetch('/file_upload', {
            method: 'POST',
            body: new FormData(uploadForm)
        })
        .then(response => response.ok)
        .then(data => {
            if (data) {
                alert("文件上传成功");
            } else {
                alert("文件上传失败");
            }
        })
    });
}

init();