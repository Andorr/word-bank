
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