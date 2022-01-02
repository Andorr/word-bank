
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

export class Word {

    id: string;
    kind: string;
    value: string;
    translations: Translation[];

    constructor(id: string, value: string, kind: string, translations: Translation[]) {
        this.id = id;
        this.value = value;
        this.kind = kind;
        this.translations = translations;
    }

    static fromObject(object: Record<string, any>) {
        return new Word(object.id, object.value, object.kind, object.translations);
    }

    toObject(): Record<string, any> {
        return {
            id: this.id,
            value: this.value,
            kind: this.kind,
            translations: this.translations
        }
    }
}

export class Folder {

    id: string;
    name: string;
    parent?: string;
    words: string[];

    constructor(id: string, name: string, words: string[], parent?: string) {
        this.id = id;
        this.name = name;
        this.parent = parent;
        this.words = words;
    }

    static fromObject(object: Record<string, any>) {
        return new Folder(object.id, object.name, object.words, object.parent);
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

export type PageResult = { 
    total: number;
    page: number;
    count: number;
    results: Word[];
}

export type WordQueryOptions = {
    query?: string;
    word?: string;
    kind?: string;
    tags?: string[];
}

export type PaginationOptions = {
    limit?: number;
    page?: number;
}