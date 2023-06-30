import './Header.css'

function toConfig() {
  console.log('to config.')
}

function Header() {
  return (
    <div className="search">
      <form>
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
        >
        </input>
      </form>
      <button onClick={toConfig}>・・・</button>
    </div>
  )
}

export default Header;
