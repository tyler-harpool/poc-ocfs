const express = require('express');
const request = require('request');
const path = require("path");

const app = express();
const API_URL = 'https://www.courtlistener.com/api/rest/v3/dockets/?format=json' // Replace this URL with your own
app.use(express.static(path.join(__dirname, 'public')));


app.use((req, res, next) => {
    res.header('Access-Control-Allow-Origin', '*');
    next();
});

app.get('/api', (req, res) => {
    request(
        { url: `${API_URL}` },
        (error, response, body) => {
            if (error || response.statusCode !== 200) {
                return res.status(500).json({ type: 'error', message: error.message });
            }

            res.json(JSON.parse(body));
        }
    );
});

app.get('/docket*', (req, res) => {
    request(
        { url: `${API_URL}` },
        (error, response, body) => {
            if (error || response.statusCode !== 200) {
                return res.status(500).json({ type: 'error', message: error.message });
            }

            res.json(JSON.parse(body));
        }
    );
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => console.log(`listening on ${PORT}`));