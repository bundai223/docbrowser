import { ChangeEvent, FormEvent, useState } from 'react';
import Button from './atom/Button';
import './Header.css'
// import { invoke } from '@tauri-apps/api';
import { tauriClient } from '@/client'

const search = (search_word: string): Promise<string> => {
	// return await invoke('get_app_name')
	return tauriClient.query(['app.search', search_word]) // エディタ補完が効く
}


function toConfig() {
  console.log('to config.')
}

type Props = {
  searchHandler?: SearchHandler
}

export type SearchResult = {
  id: number;
  doctype: string;
  html_path: string;
  name: string;
}
export type SearchHandler = (searched: SearchResult[]) => void

function Header(props: Props) {
  // console.log(props)
  const [ search_query, setSearchQuery ] = useState<string>('')

  function onChange(e: ChangeEvent<HTMLInputElement>) {
    const _query = e.currentTarget.value;
    setSearchQuery(_query);

    search(_query).then(a => {
      console.log(a)
    });

    // invoke('search', { word: _query }).then((docsets) => {
    //   console.log(docsets)
      // if (docsets instanceof Array<string>) {
      //   const result: SearchResult[] = docsets.map((word: string) => { return { word: word } })
      //   if (props.searchHandler) {
      //     props.searchHandler(result)
      //   }
      // }
    // })
  }

  function onSubmit(e: FormEvent) {
    e.preventDefault();
  }


  return (
    <div className="header">
      <form className="search" onSubmit={onSubmit}>
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
          onChange={onChange}
        >
        </input>
      </form>
      <Button onClick={toConfig}>・・・</Button>
    </div>
  )
}

export default Header;
