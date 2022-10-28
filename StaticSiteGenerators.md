# Some tools to create a static site from your markdown documentation


# [mdBook](https://github.com/rust-lang/mdBook)

mdBook is a utility to create modern online books from Markdown files.

[**mdBook**](https://rust-lang.github.io/mdBook/) is a command line tool to create books with Markdown. It is ideal for creating product or API documentation, tutorials, course materials or anything that requires a clean, easily navigable and customizable presentation.

-   Lightweight [Markdown](https://rust-lang.github.io/mdBook/format/markdown.html) syntax helps you focus more on your content
-   Integrated [search](https://rust-lang.github.io/mdBook/guide/reading.html#search) support
-   Color [syntax highlighting](https://rust-lang.github.io/mdBook/format/theme/syntax-highlighting.html) for code blocks for many different languages
-   [Theme](https://rust-lang.github.io/mdBook/format/theme/index.html) files allow customizing the formatting of the output
-   [Preprocessors](https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html) can provide extensions for custom syntax and modifying content
-   [Backends](https://rust-lang.github.io/mdBook/format/configuration/renderers.html) can render the output to multiple formats
-   Written in [Rust](https://www.rust-lang.org/) for speed, safety, and simplicity
-   Automated testing of [Rust code samples](https://rust-lang.github.io/mdBook/cli/test.html)

https://github.com/rust-lang/mdBook




# [Zola](https://www.getzola.org/)

Zola is a static site generator (SSG), similar to [Hugo](https://gohugo.io/), [Pelican](https://blog.getpelican.com/), and [Jekyll](https://jekyllrb.com/) It is written in [Rust](https://www.rust-lang.org/) and uses the [Tera](https://tera.netlify.com/) template engine, which is similar to [Jinja2](https://jinja.palletsprojects.com/en/2.10.x/), [Django templates](https://docs.djangoproject.com/en/2.2/topics/templates/), [Liquid](https://shopify.github.io/liquid/), and [Twig](https://twig.symfony.com/). Content is written in [CommonMark](https://commonmark.org/), a strongly defined, highly compatible specification of [Markdown](https://www.markdownguide.org/).

SSGs use dynamic templates to transform content into static HTML pages. Static sites are thus very fast and require no databases, making them easy to host.

### Features

-   Single binary
-   Syntax highlighting
-   Sass compilation
-   Assets co-location
-   (Basic currently) multilingual site suport
-   Image processing
-   Themes
-   Shortcodes
-   Internal links
-   External link checker
-   Table of contents automatic generation
-   Automatic header anchors
-   Aliases
-   Pagination
-   Custom taxonomies
-   Search with no servers or any third parties involved
-   Live reload
-   Deploy on many platforms easily: Netlify, Vercel, Cloudflare
-   Breadcrumbs

https://github.com/getzola/zola
https://www.getzola.org/




# [Cobalt](https://cobalt-org.github.io/)
A straightforward static site generator written in [Rust](http://www.rust-lang.org/).
**Simple:** Cobalt will help you get up and running quickly but scales to handle your more complex cases
**Easy:** Cobalt tries to be no-hassle, from being easy to install, a workflow-centric command line, to a familiar template language.
**Fast:** Written in Rust, Cobalt is blazing fast and allows you to quickly see the results of your work.

https://github.com/cobalt-org/cobalt.rs
https://cobalt-org.github.io/




# [MkDocs](https://www.mkdocs.org/)

MkDocs is a **fast**, **simple** and **downright gorgeous** static site generator that's geared towards building project documentation. Documentation source files are written in Markdown, and configured with a single YAML configuration file. Start by reading the [introductory tutorial](https://www.mkdocs.org/getting-started/), then check the [User Guide](https://www.mkdocs.org/user-guide/) for more information.

**Feautures:** Great themes available, Easy to customize, Preview your site as you work, Host anywhere

**Requirements:** MkDocs requires a recent version of [Python](https://www.python.org/) and the Python package manager, [pip](https://pip.readthedocs.io/en/stable/installing/), to be installed on your system. 
MkDocs comes with a built-in dev-server that lets you preview your documentation as you work on it. Make sure you're in the same directory as the `mkdocs.yml` configuration file, and then start the server by running the `mkdocs serve` command:
https://www.mkdocs.org/getting-started/

**Deploying:** GitHub Pages, Organization and User Pages, Custom Domains
https://www.mkdocs.org/user-guide/deploying-your-docs/

https://www.mkdocs.org/




# [VuePress](https://vuepress.vuejs.org/)

**Simplicity First:** Minimal setup with markdown-centered project structure helps you focus on writing.
**Vue-Powered:** Enjoy the dev experience of Vue + webpack, use Vue components in markdown, and develop custom themes with Vue.
**Performant:** VuePress generates pre-rendered static HTML for each page, and runs as an SPA once a page is loaded.

**Prerequisites:** `Node.js 10+, Yarn Classic`

**Feautures:** Built-in Markdown extensions, Using Vue in Markdown, Vue-powered custom theme system, Plugins

VuePress is composed of two parts: a [minimalistic static site generator](https://github.com/vuejs/vuepress/tree/master/packages/%40vuepress/core) with a Vue-powered [theming system](https://vuepress.vuejs.org/theme/) and [Plugin API](https://vuepress.vuejs.org/plugin/), and a [default theme](https://vuepress.vuejs.org/theme/default-theme-config.html) optimized for writing technical documentation. It was created to support the documentation needs of Vue’s own sub projects.

Each page generated by VuePress has its own pre-rendered static HTML, providing great loading performance and is SEO-friendly. Yet, once the page is loaded, Vue takes over the static content and turns it into a full Single-Page Application (SPA). Extra pages are fetched on demand as the user navigates around the site

A VuePress site is in fact a SPA powered by `Vue, Vue Router` and `webpack`. If you’ve used Vue before, you will notice the familiar development experience when you are writing or developing custom themes (you can even use Vue DevTools to debug your custom theme!). 
During the build, we create a server-rendered version of the app and render the corresponding HTML by virtually visiting each route. This approach is inspired by `Nuxt's` nuxt generate command and other projects like `Gatsby`
Each Markdown file is compiled into HTML with `markdown-it` and then processed as the template of a Vue component. This allows you to directly use Vue inside your Markdown files and is great when you need to embed dynamic content.
https://vuepress.vuejs.org/guide/

### [Why Not ...?](https://vuepress.vuejs.org/guide/#why-not)

### Nuxt
Nuxt is capable of doing what VuePress does, but it’s designed for building applications. VuePress is focused on content-centric static sites and provides features tailored for technical documentation out of the box.

### Docsify / Docute
Both are great projects and also Vue-powered. Except they are both fully runtime-driven and therefore not SEO-friendly. If you don’t care for SEO and don’t want to mess with installing dependencies, these are still great choices.

### Hexo
Hexo has been serving the Vue docs well - in fact, we are probably still a long way to go from migrating away from it for our main site. The biggest problem is that its theming system is static and string-based - we want to take advantage of Vue for both the layout and the interactivity. Also, Hexo’s Markdown rendering isn’t the most flexible to configure.

### GitBook
We’ve been using GitBook for most of our sub project docs. The primary problem with GitBook is that its development reload performance is intolerable with a large amount of files. The default theme also has a pretty limiting navigation structure, and the theming system is, again, not Vue based. The team behind GitBook is also more focused on turning it into a commercial product rather than an open-source tool.

https://vuepress.vuejs.org/




# [HUGO](https://gohugo.io/)

The world’s fastest framework for building websites. Hugo is one of the most popular open-source static site generators. With its amazing speed and flexibility, Hugo makes building websites fun again. 

Hugo is a static HTML and CSS website generator written in [Go](https://go.dev/). It is optimized for speed, ease of use, and configurability. Hugo takes a directory with content and templates and renders them into a full HTML website.

Hugo relies on Markdown files with front matter for metadata, and you can run Hugo from any directory. This works well for shared hosts and other systems where you don’t have a privileged account.

Hugo renders a typical website of moderate size in a fraction of a second. A good rule of thumb is that each piece of content renders in around 1 millisecond.

Hugo is designed to work well for any kind of website including blogs, tumbles, and docs.

Because Hugo renders _static_ websites, you can host your new Hugo website virtually anywhere. The following represent only a few of the more popular hosting and automated deployment solutions used by the Hugo community.

https://gohugo.io/




# [GitBook](https://www.gitbook.com/)

GitBook is a modern documentation platform where teams can document everything from products to internal knowledge bases and APIs.

**Write & Publish with Markdown and Git**
Books on GitBook are written using a beautiful editor and published using Git or GitHub.

**Distribute & Sell everywhere**
Sell your book at the price you want, GitBook can also distribute it on all main marketplaces.

**Analyse & Engage your readers**
Learn from your readers using powerful analytics, and engage with them to improve your book's content.

https://www.gitbook.com/




## [Catalog](https://jamstack.org/generators/)

