const URLS = {
    tabs: '/tabs',

    words: '/words',
    wordsUpsert: '/upsert',
    wordsSearch: '/search',

    folders: '/folders',
    foldersUpsert: '/upsert',
}
export default URLS;

const wordUpsert = (id?: string, parentId?: string) => {
    const path = URLS.tabs.concat(URLS.words, URLS.wordsUpsert);
    const query = Object.entries({ id: id , parent: parentId })
        .filter(([, value]) => value)
        .map(([key, value]) => `${key}=${value}`).join("&");
    return path + '?' + query;
}

const folderUpsert = (id?: string, parentId?: string) => {
    const path = URLS.tabs.concat(URLS.folders, URLS.foldersUpsert);
    const query = Object.entries({ id: id , parent: parentId })
        .filter(([, value]) => value)
        .map(([key, value]) => `${key}=${value}`).join("&");
    return path + '?' + query;
}

export const PATHS = {
    wordUpsert,
    folderUpsert,
}