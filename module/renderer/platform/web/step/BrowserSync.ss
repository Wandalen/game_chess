let browserSync = require( 'browser-sync' ).create();
let path = require( 'path' );
let rootPath = path.join( __dirname, '../../../../..' );

let o =
{
  open: 'local',
  server:
  {
    baseDir : path.join( rootPath, 'target/web' ),
    index: 'index.html'
  },
  startPath : 'index.html',
  watch: true,
  ui: false
}

let inited = false;
let watcher = null;
let watchEventsCounter = 0;

function watcherCb()
{
  watchEventsCounter++;

  if( inited || watchEventsCounter < 2 )
  return;

  inited = true;
  watcher.close();
  browserSync.init( o );
}

watcher = browserSync.watch( path.join( o.server.baseDir, '*.wasm' ) );
watcher.on( 'add', watcherCb );
watcher.on( 'change', watcherCb );

