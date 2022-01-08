const URLS = {
    tabs: '/tabs',

    landing: '/landing',

    words: '/words',
    wordsUpsert: '/upsert',
    wordsSearch: '/search',

    folders: '/folders',
    foldersUpsert: '/upsert',

    quiz: '/quiz',
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

const quizLanding = (): string => {
    return URLS.quiz;
}

const quiz = (id: string): string => {
    return URLS.quiz.concat('/', id);
}

export const PATHS = {
    wordUpsert,
    folderUpsert,
    quizLanding,
    quiz,
}