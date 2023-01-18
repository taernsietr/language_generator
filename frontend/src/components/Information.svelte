<script>
    export let language = "enUS";
    let hidden = false;
    let text = 
        {intro_text: 
            {
                enUS: ["<p>This (at the moment, at least) is a simple word and text generator. It is based on <i>categories</i>, which are just sets of unicode characters without any deeper meaning, and <i>patterns</i>, which represent syllable patterns - but similarly to the categories, they are treated here as simple character patterns. As such, you can create as many categories as you want, with whichever characters or groups thereof, and set up patterns so that they appear in word generation.</p>",
                "<p>TL;DR:",
                "<ul>",
                    "<li>words are generated according to the syllable patterns;</li>",
                    "<li>patterns are composed of the symbols for each category;</li>",
                    "<li>each category is made of up groups of characters;</li>",
                    "<li>You can have several categories or patterns.</li>",
                "</ul>",
                "</p>"],
                ptBR: ["<p>Esse programa é (até então) um gerador de palavras e texto simples. Ele é baseado em <i>categorias</i>, que são conjuntos de caracteres unicode sem qualquer semântica, e <i>padrões</i>, que representam padrões silábicos; mas, assim como as categorias, são apenas padrões simples de caracteres. Dessa forma, você pode criar quantas categorias que quiser, com quantos caracteres ou conjuntos de caracteres quiser, e configurar os padrões para que apareçam na geração de palavras.</p>",
                "<p>TL;DR:",
                "<ul>",
                    "<li>palavras são geradas de acordo com os padrões silábicos;</li>",
                    "<li>padrões são compostos pelos símbolos de cada categoria;</li>",
                    "<li>cada categoria é composta de grupos de caracteres;</li>",
                    "<li>você pode ter várias categorias e padrões.</li>",
                "</ul>",
                "</p>"]
            }
        };
</script>

{#if !hidden}
<div id="infoDisplay">
        <h3>Information</h3>
        <div id="infoContents">
            {@html text.intro_text[language].join("\n")}
        </div>
</div>
{/if}

