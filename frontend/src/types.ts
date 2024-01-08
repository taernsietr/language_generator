interface CategoryType {
    symbol: string;
    elements: string[];
}

interface PatternType {
    pattern: string;
    position: string;
    weight: string;
}

interface RuleType {
    context: string;
    result: string;
}

interface GeneratorSettings {
    name: string;
    categories: CategoryType[];
    patterns: PatternType[];
    ruleset: RuleType[];
}

export type { CategoryType, PatternType, RuleType, GeneratorSettings }
