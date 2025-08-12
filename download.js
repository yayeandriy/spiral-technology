// download.js
document.addEventListener("DOMContentLoaded", function() {
    window.downloadImagesZip = async function() {
        const zip = new JSZip();
        // Create a folder inside the ZIP for the images
        const imgFolder = zip.folder("images");
        
        // Select all <img> elements on the page
        const images = document.querySelectorAll("img");
        const promises = [];
    
        images.forEach((img, index) => {
            const url = img.src;
            const promise = fetch(url)
                .then(response => {
                    if (!response.ok) {
                        throw new Error(`Failed to fetch image: ${url}`);
                    }
                    return response.blob();
                })
                .then(blob => {
                    // You can determine a better filename and extension if needed.
                    const fileName = `image${index + 1}.jpg`;
                    imgFolder.file(fileName, blob);
                })
                .catch(err => console.error(err));
            promises.push(promise);
        });
    
        // Wait for all images to be added to the ZIP
        await Promise.all(promises);
    
        // Generate the ZIP file and trigger a download
        zip.generateAsync({ type: "blob" }).then(function(content) {
            saveAs(content, "images.zip");
        });
    }
});
