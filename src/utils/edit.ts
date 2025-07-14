import hljs from 'highlight.js/lib/core'
import javascript from 'highlight.js/lib/languages/javascript'
import typescript from 'highlight.js/lib/languages/typescript'
import python from 'highlight.js/lib/languages/python'
import json from 'highlight.js/lib/languages/json'
import xml from 'highlight.js/lib/languages/xml'
import html from 'highlight.js/lib/languages/xml' // html 用 xml
import css from 'highlight.js/lib/languages/css'
import shell from 'highlight.js/lib/languages/shell'
import markdown from 'highlight.js/lib/languages/markdown'
import yaml from 'highlight.js/lib/languages/yaml'
import c from 'highlight.js/lib/languages/c'
import cpp from 'highlight.js/lib/languages/cpp'
import java from 'highlight.js/lib/languages/java'
import go from 'highlight.js/lib/languages/go'
import rust from 'highlight.js/lib/languages/rust'
import php from 'highlight.js/lib/languages/php'
import bash from 'highlight.js/lib/languages/bash'
import sql from 'highlight.js/lib/languages/sql'

const registerLanguage = () => {
    hljs.registerLanguage('javascript', javascript)
    hljs.registerLanguage('typescript', typescript)
    hljs.registerLanguage('python', python)
    hljs.registerLanguage('json', json)
    hljs.registerLanguage('xml', xml)
    hljs.registerLanguage('html', html)
    hljs.registerLanguage('css', css)
    hljs.registerLanguage('shell', shell)
    hljs.registerLanguage('markdown', markdown)
    hljs.registerLanguage('yaml', yaml)
    hljs.registerLanguage('c', c)
    hljs.registerLanguage('cpp', cpp)
    hljs.registerLanguage('java', java)
    hljs.registerLanguage('go', go)
    hljs.registerLanguage('rust', rust)
    hljs.registerLanguage('php', php)
    hljs.registerLanguage('bash', bash)
    hljs.registerLanguage('sql', sql)
}

export { registerLanguage }