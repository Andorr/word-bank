
export class Translation {
    id: string;
    value: string;

    constructor(id: string, value: string) {
        this.id = id;
        this.value = value;
    }

    toObject(): Record<string, any> {
        return {
            id: this.id,
            value: this.value,
        }
    }
}

export interface WordObject {
    id: string;
    class: string;
    value: string;
    translations: Translation[];
}


export class Word {

    id: string;
    classType: string;
    value: string;
    translations: Translation[];

    constructor(id: string, value: string, classType: string, translations: Translation[]) {
        this.id = id;
        this.value = value;
        this.classType = classType;
        this.translations = translations;
    }

    static fromObject(object: WordObject): Word {
        return new Word(object.id, object.value, object.class, object.translations);
    }

    toObject(): WordObject {
        return {
            id: this.id,
            value: this.value,
            class: this.classType,
            translations: this.translations
        }
    }
}

export class Folder {

    id: string;
    name: string;
    parent?: string;
    words: string[];

    createdAt: string;

    constructor(id: string, name: string, words: string[], parent?: string, createdAt?: string) {
        this.id = id;
        this.name = name;
        this.parent = parent;
        this.words = words;
        this.createdAt = createdAt || new Date().toISOString();
    }

    static fromObject(object: Record<string, any>) {
        return new Folder(object.id, object.name, object.words, object.parent, object.createdAt);
    }

    update(f: Folder) {
        if(f.id) {
            this.id = f.id;
        }
        if(f.name) {
            this.name = f.name;
        }
        if(f.parent) {
            this.parent = f.parent;
        }
        if(f.words) {
            this.words = f.words;
        }
    }

    toObject(): Record<string, any> {
        return {
            id: this.id,
            name: this.name,
            parent: this.parent,
            words: this.words,
        }
    }
}

export type FolderResult = {
    data: Folder;
    content: {
        folders: Folder[];
        words: Word[];
    };
}

export type PageResult<T> = { 
    total: number;
    page: number;
    count: number;
    results: T[];
}

export type WordQueryOptions = {
    query?: string;
    word?: string;
    class?: string;
    tags?: string[];
}

export type FolderQueryOptions = {
    query?: string;
    words?: string[];
    parent?: string;
}

export type PaginationOptions = {
    limit?: number;
    page?: number;
}

export enum QuizMode {
    Normal = "Normal",
    Endless = "Endless",
}

export enum QuizQuestionPolicy {
    WordToTranslations,
    TranslationsToWord,
    Random,
}

export type QuizWordOption = {
    folders?: string[];
    count?: number;
}

export type QuizOptions = {
    mode: QuizMode;
    policy?: QuizQuestionPolicy;
    words: QuizWordOption;
}

export type Quiz = {
    id: string;
    words: Word[];
    options: QuizOptions;
}

export type QuizResult = {
    id: string;
    results: QuizWordResults[];
    createdAt: string;
}

export type QuizWordResults = {
    wordId: string;
    numCorrects: number;
    numIncorrects: number;
}