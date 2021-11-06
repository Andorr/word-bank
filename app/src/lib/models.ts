
export class Translation {
    id: string;
    value: string;

    constructor(id: string, value: string) {
        this.id = id;
        this.value = value;
    }
}

export class Word {

    id: string;
    value: string;
    translations: Translation[];

    constructor(id: string, value: string, translations: Translation[]) {
        this.id = id;
        this.value = value;
        this.translations = translations;
    }

    static fromObject(object: Record<string, any>) {
        return new Word(object.id, object.value, object.translations);
    }
}

export type PageResult = { 
    total: number;
    page: number;
    count: number;
    results: Word[];
}