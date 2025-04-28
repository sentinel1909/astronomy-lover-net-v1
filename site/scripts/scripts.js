// scripts.js

// the 'use strict' directive enforces stricter parsing and error handling on the code at runtime
'use strict';

// add the current year to the page footer
let currentYear = new Date().getFullYear();
let year = document.getElementById('year');
year.textContent = currentYear;