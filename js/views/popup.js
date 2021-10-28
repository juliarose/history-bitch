'use strict';

import { readyState } from '../app/readyState.js';
import { tabs } from '../app/browser.js';

const page = {
    buttons: {
        index: document.getElementById('tabs.index')
    }
};

async function onApp(app) {
    // add listeners
    (function() {
        page.buttons.index.addEventListener('click', () => {
            tabs.create({
                url: '/views/index2.html'
            });
        });
    }());
}

// ready
readyState().then(onApp);