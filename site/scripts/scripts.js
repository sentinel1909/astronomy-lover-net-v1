// scripts.ts
// the 'use strict' directive enforces stricter parsing and error handling on the code at runtime
'use strict';
function renderApod(data) {
    const title = document.getElementById('title');
    const date = document.getElementById('date');
    const explanation = document.getElementById('explanation');
    const image = document.getElementById('image');
    const video = document.getElementById('video');
    if (title)
        title.textContent = data.title;
    if (date)
        date.textContent = data.date;
    if (explanation)
        explanation.textContent = data.explanation;
    if (data.media_type === 'image' && image) {
        image.src = data.url;
        image.alt = data.title;
        image.style.display = 'block';
    }
    else if (data.media_type === 'video' && video) {
        video.src = data.url;
        video.style.display = 'block';
        image.style.display = 'none'; // hide image
    }
    else {
        // fallback if media_type is unknown
        image.style.display = 'none';
        video.style.display = 'none';
    }
}
fetch('/fetch')
    .then((res) => res.json())
    .then((response) => {
    renderApod(response.content);
})
    .catch((err) => {
    console.error('Failed to load APOD:', err);
});
// add the current year to the page footer
let currentYear = new Date().getFullYear();
let year = document.getElementById('year');
if (year) {
    year.textContent = currentYear.toString();
}
