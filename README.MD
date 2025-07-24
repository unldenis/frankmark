# frankmark

A modern documentation generator with FrankenUI styling, built in Rust.

## TODO List

### 🔥 High Priority (Core Functionality)

#### Markdown Processing
- [ ] **Add markdown-to-HTML conversion** - Currently displays raw markdown text
- [ ] **Add markdown parser dependency** (pulldown-cmark or markdown crate)
- [ ] **Support GitHub Flavored Markdown** (GFM) features
- [ ] **Syntax highlighting for code blocks**
- [ ] **Math/LaTeX support** for mathematical expressions

#### Navigation & Structure
- [ ] **Breadcrumb navigation** - Show current page path
- [ ] **Previous/Previous page navigation** - Currently only has "Next"
- [ ] **Table of Contents (TOC)** - Auto-generate from headings
- [ ] **Page ordering** - Support custom page order within folders
- [ ] **Nested folder support** - Currently only supports one level

#### Search Functionality
- [ ] **Implement search backend** - Currently search input is non-functional
- [ ] **Generate search index** - Create JSON index of all content
- [ ] **Client-side search** - JavaScript search implementation
- [ ] **Search result highlighting**

### 🎨 Medium Priority (UX & Features)

#### Content Enhancement
- [ ] **Custom page titles** - Support title metadata in markdown
- [ ] **Meta descriptions** - For SEO
- [ ] **Custom CSS support** - Allow user-defined stylesheets
- [ ] **Image optimization** - Responsive images, lazy loading
- [ ] **Code copy button** - One-click copy for code blocks
- [ ] **Anchor links** - Clickable section headers

#### Configuration & Customization
- [ ] **Site title configuration** - Currently hardcoded "Test Page"
- [ ] **Custom logo/branding** - Support for custom header content
- [ ] **Multiple theme support** - Beyond just FrankenUI
- [ ] **Custom templates** - Allow users to provide custom HTML templates
- [ ] **Plugin system** - Extensible architecture

#### Performance & Build
- [ ] **Incremental builds** - Only rebuild changed files
- [ ] **Build optimization** - Minify CSS/JS, optimize images
- [ ] **Development server** - Live reload during development
- [ ] **Build time optimization** - Parallel processing

### 🚀 Low Priority (Advanced Features)

#### Advanced Content Features
- [ ] **Mermaid diagrams** - Support for flowcharts, diagrams
- [ ] **Interactive examples** - Code playgrounds
- [ ] **Versioning support** - Multiple documentation versions
- [ ] **Multi-language support** - Internationalization
- [ ] **PDF export** - Generate PDF versions of pages

#### Collaboration & Workflow
- [ ] **Git integration** - Show last modified, contributors
- [ ] **Comment system** - Allow user feedback on pages
- [ ] **Edit on GitHub** - Direct links to edit source files
- [ ] **Version control integration** - Show git history

#### Analytics & SEO
- [ ] **Google Analytics integration**
- [ ] **Sitemap generation**
- [ ] **RSS feeds**
- [ ] **Social media meta tags**

### 🔧 Technical Improvements

#### Architecture
- [ ] **CLI interface** - Command-line arguments for different operations
- [ ] **Configuration validation** - Better error messages for invalid config
- [ ] **Error handling improvements** - More descriptive error messages
- [ ] **Logging system** - Structured logging for debugging
- [ ] **Testing suite** - Unit and integration tests

#### Template System
- [ ] **Template inheritance** - Base templates with blocks
- [ ] **Custom filters** - Template filters for content processing
- [ ] **Asset pipeline** - CSS/JS bundling and optimization
- [ ] **Template caching** - Improve build performance

### 📊 Comparison with GitBook/mdBook

#### What frankmark has:
- ✅ Modern FrankenUI styling
- ✅ Dark/light mode support
- ✅ Responsive design
- ✅ Theme customization
- ✅ Basic navigation structure

#### What's missing compared to GitBook:
- ❌ Rich text editor
- ❌ Real-time collaboration
- ❌ Advanced search
- ❌ Custom domains
- ❌ Analytics dashboard
- ❌ User management

#### What's missing compared to mdBook:
- ❌ Comprehensive markdown support
- ❌ Rust-specific features (doc tests, etc.)
- ❌ Extensive plugin ecosystem
- ❌ Advanced configuration options
- ❌ Better error reporting

### 🎯 Next Steps (Immediate)

1. **Add markdown parsing** - This is the most critical missing piece
2. **Implement search** - Make the search input functional
3. **Add breadcrumbs** - Improve navigation UX
4. **Support page titles** - Dynamic page titles from content
5. **Add TOC generation** - Auto-generate table of contents

### 🛠️ Development Setup

```bash
# Build the project
cargo build

# Run the generator
cargo run

# Check the output
open output/Introduction/Hello-World.html
```