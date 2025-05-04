// scripts.ts

// the 'use strict' directive enforces stricter parsing and error handling on the code at runtime
'use strict';

// type to represent fetched data from the NASA APOD API
interface NasaData {
  date: string,
  title: string,
  explanation: string,
  url: string,
  copyright: string,
}

// add the current year to the page footer
let currentYear = new Date().getFullYear();
let year = document.getElementById('year');

if (year) {
  year.textContent = currentYear.toString();
}
