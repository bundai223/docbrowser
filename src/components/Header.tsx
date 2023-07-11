import { FormEvent, useState } from 'react';
import Button from './atom/Button';
import './Header.css'
import { invoke } from '@tauri-apps/api';

function toConfig() {
  console.log('to config.')
}

type Props = {
  searchHandler?: SearchHandler
}

export type SearchResult = {
  word: string;
}
export type SearchHandler = (searched: SearchResult[]) => void

function Header(props: Props) {
  // console.log(props)
  const [ search_query, setSearchQuery ] = useState<string>('')

  function onSubmit(e: FormEvent) {
    if (e.target && e.target instanceof HTMLFormElement) {
      e.preventDefault();
      
      // console.log(search_query)

      invoke('search', { word: search_query }).then((docsets) => {
        if(docsets instanceof Array<string>) {
          const result: SearchResult[] = docsets.map((word: string) => { return { word: word } })
          if (props.searchHandler) {
            props.searchHandler(result)
          }
        }
      })
    }
  }

  return (
    <div className="search">
      <form onSubmit={onSubmit}>
        <input type="search"
          name="q"
          className="_search-input"
          placeholder="Search…"
          autoComplete="off"
          autoCapitalize="off"
          autoCorrect="off"
          spellCheck="false"
          maxLength={30}
          aria-label="Search"
          onChange={(event) => setSearchQuery(event.currentTarget.value)}
        >
        </input>
      </form>
      <Button onClick={toConfig}>・・・</Button>
    </div>
  )
}

export default Header;
