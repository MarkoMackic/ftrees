import Page from '../components/Page.svelte';

// import the global css
import '../css/global.scss';

const load_page = function(cmp, cmpProps = {}, title = 'FTrees') {

    // we'll hold init code here .. like loading localizations, or we'll do it at the page level 

    return new Page({target: document.body, props: {
            cmp,
            cmpProps,
            title
    }});
}


export {
    load_page
}