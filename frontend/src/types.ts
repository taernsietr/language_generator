interface CategoryType {
    symbol: string;
    elements: string[];
}

interface PatternType {
    pattern: string;
    position: string;
    weight: string;
}

interface GeneratorSettings {
    name: string;
    categories: CategoryType[];
    patterns: PatternType[];
}

export type { CategoryType, PatternType, GeneratorSettings }
