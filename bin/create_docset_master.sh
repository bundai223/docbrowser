#!/bin/bash
set -uex

insert() {
    name=$1
    alias=$2
    feed_url=$3
    docset_path=$4
    downloaded=0

    sqlite3 $file "insert into docsets (name, alias, feed_url, docset_path, downloaded) values('$name', '$alias', '$feed_url', '$docset_path', '$downloaded');"
}

# https://github.com/Kapeli/feeds
# 一番下で
docset_base_path="./docsets"
file="$docset_base_path/docsets.sqlite3"

rm -f $file
mkdir -p $docset_base_path

sqlite3 $file "create table docsets(id integer primary key autoincrement, name text, alias text, feed_url text, docset_path text, downloaded boolean);"

# sqlite3 $file "insert into docsets (name, alias, feed_url, docset_path, downloaded) values('TypeScript', 'ts', 'https://raw.githubusercontent.com/Kapeli/feeds/master/TypeScript.xml', 'TypeScript.docset', 1);"
insert 'TypeScript' 'ts' 'https://raw.githubusercontent.com/Kapeli/feeds/master/TypeScript.xml' 'TypeScript.docset'
insert 'Rust' 'rs' 'https://raw.githubusercontent.com/Kapeli/feeds/master/Rust.xml' 'Rust.docset'
insert 'Ruby' 'rb' 'https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby_3.xml' 'Ruby.docset'
insert 'Bash' 'bash' 'https://raw.githubusercontent.com/Kapeli/feeds/master/Bash.xml' 'Bash.docset'
insert 'CSS' 'css' 'https://raw.githubusercontent.com/Kapeli/feeds/master/CSS.xml' 'CSS.docset'
insert 'Docker' 'docker' 'https://raw.githubusercontent.com/Kapeli/feeds/master/Docker.xml' 'Docker.docset'

#https://raw.githubusercontent.com/Kapeli/feeds/master/AWS_JavaScript.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/ActionScript.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Akka.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Android.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Angular.dart.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Angular.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/AngularJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/AngularTS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ansible.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Apache_HTTP_Server.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Appcelerator_Titanium.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/AppleScript.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Apple_Guides_and_Sample_Code.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Arduino.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/BackboneJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Bash.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Boost.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Bootstrap_2.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Bootstrap_3.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Bootstrap_4.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Bootstrap_5.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Bourbon.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/C++.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/C.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/CMake.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/CSS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/CakePHP.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Cappuccino.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Chai.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Chef.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Clojure.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Cocos2D-X.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Cocos2D.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Cocos3D.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/CodeIgniter.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/CoffeeScript.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/ColdFusion.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Common_Lisp.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Compass.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Cordova.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Corona.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/CouchDB.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Craft.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/D3JS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Dart.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Django.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Docker.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Doctrine_ORM.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Dojo.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Drupal_10.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Drupal_7.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Drupal_8.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Drupal_9.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/ElasticSearch.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Elixir.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Emacs_Lisp.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/EmberJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Emmet.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Erlang.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Express.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/ExpressionEngine.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/ExtJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Flask.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Font_Awesome.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Foundation.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/GLib.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Go.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Gradle_DSL.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Gradle_Groovy_API.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Gradle_Java_API.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Gradle_User_Guide.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Grails.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Groovy.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Groovy_JDK.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Grunt.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Gulp.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/HTML.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/HTTP.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Haml.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Handlebars.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Haskell.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ionic.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Jade.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Jasmine.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/JavaFX.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/JavaScript.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_EE6.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_EE7.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_EE8.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE10.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE11.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE12.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE13.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE14.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE15.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE16.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE17.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE18.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE19.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE20.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE6.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE7.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE8.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Java_SE9.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Jekyll.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Jinja.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Joomla.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Julia.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/KnockoutJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Kobold2D.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/LaTeX.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Laravel.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Less.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Lo-Dash.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Lua_5.1.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Lua_5.2.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Lua_5.3.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Lua_5.4.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/MATLAB.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Man_Pages.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/MarionetteJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Markdown.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Matplotlib.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Meteor.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Mocha.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/MomentJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/MongoDB.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Mongoose.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Mono.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/MooTools.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/MySQL.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/NET_Framework.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Neat.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Nginx.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/NodeJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/NumPy.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/OCaml.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/OpenCV.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/OpenCV_C++.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/OpenCV_C.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/OpenCV_Java.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/OpenCV_Python.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/OpenGL_2.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/OpenGL_3.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/OpenGL_4.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/PHP.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/PHPUnit.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Pandas.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Perl.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Phalcon.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/PhoneGap.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Play_Java.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Play_Scala.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Polymer.dart.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/PostgreSQL.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Processing.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/PrototypeJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Pug.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Puppet.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Python_2.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Python_3.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Qt_4.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Qt_5.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Qt_6.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/R.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/README.md
#https://raw.githubusercontent.com/Kapeli/feeds/master/Racket.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/React.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Redis.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/RequireJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/RubyMotion.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby_2.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby_3.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby_Installed_Gems.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby_on_Rails_3.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby_on_Rails_4.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby_on_Rails_5.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby_on_Rails_6.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Ruby_on_Rails_7.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Rust.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/SQLAlchemy.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/SQLite.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/SVG.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/SailsJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/SaltStack.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Sass.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Scala.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/SciPy.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Semantic_UI.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Sencha_Touch.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Sinon.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Smarty.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Sparrow.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Spring_Framework.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Statamic.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Stylus.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Susy.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Swift.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Symfony.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/TYPO3.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Tcl.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Tornado.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Twig.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Twisted.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/TypeScript.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/UnderscoreJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Unity_3D.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/VMware_vSphere.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Vagrant.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Vim.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/VueJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/WordPress.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/XSLT.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/XUL.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Xamarin.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Xojo.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/YUI.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Yii.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Zend_Framework_1.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Zend_Framework_2.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/Zend_Framework_3.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/ZeptoJS.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/jQuery.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/jQuery_Mobile.xml
#https://raw.githubusercontent.com/Kapeli/feeds/master/jQuery_UI.xml

# docset本体が存在していれば、downloadedを1にする
update_state() {
    docset_name=$1
    ls $docset_base_path/$1 > /dev/null 2>&1 && echo $1
    ls $docset_base_path/$1 > /dev/null 2>&1 && sqlite3 $file "update docsets set downloaded = 1 where docset_path = '$docset_name';"
}
export -f update_state

export docset_base_path
export file

sqlite3 $file "select docset_path from docsets;" | xargs -I% bash -cxe 'update_state %'
sqlite3 $file "select * from docsets;"