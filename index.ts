import fs from 'fs'
import postcss from 'postcss'

import * as wasm from './pkg/wasm_fastcss'
import Timer from './timer'

const filterKeys = (obj: object, pred: (key: string) => boolean) => 
  Object.keys(obj).filter(pred).reduce((acc, curr) => ({ ...acc, [curr]: (obj as Record<string, unknown>)[curr] }), {})

const run = async () => {
  const content = await fs.promises.readFile('result.css', 'utf-8')

  const parseTimer = new Timer('parse')
  const root = wasm.parse(content)
  parseTimer.end()
  console.log(root.nodes.length)

  const postcssTimer = new Timer('postcss')
  const postcssRoot = postcss.parse(content)
  postcssTimer.end()

  console.log(postcssRoot.nodes.length)
  console.log(root.nodes[2])
  console.log(filterKeys(postcssRoot.nodes[2], (key) => !['parent', 'source'].includes(key)))
}

run()