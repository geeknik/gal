# GAL Documentation Platform

Revolutionary documentation platform for the Gödelian Actor Language (GAL) featuring interactive visualizations, real-time code execution, and accessibility-first design.

## 🚀 Features

### Interactive Documentation
- **Live Code Playground**: Real-time code editing with GAL syntax highlighting
- **Visual Execution Flow**: Step-through code execution with variable inspection  
- **Interactive Visualizations**: Dynamic diagrams for actor systems and chaos engineering
- **Accessibility-First**: WCAG 2.1 AAA compliance with screen reader optimization

### Advanced Capabilities
- **Gödelian Visualizations**: Interactive self-modifying code demonstrations
- **Chaos Engineering Dashboard**: Real-time fault injection and resilience testing
- **Performance Metrics**: Live system monitoring and optimization insights
- **Offline Support**: Service worker enables offline documentation access

### Modern Architecture
- **Next.js 14**: React-based static site generation
- **TypeScript**: Type-safe development
- **Tailwind CSS**: Utility-first styling with custom design system
- **Framer Motion**: Smooth animations and interactions
- **Monaco Editor**: VSCode-quality code editing experience

## 🏃‍♂️ Quick Start

### Prerequisites
- Node.js 18+ 
- npm 8+

### Installation

```bash
# Clone the repository
git clone https://github.com/geeknik/gal.git
cd gal/docs

# Install dependencies
npm install

# Start development server
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to view the documentation platform.

### Build for Production

```bash
# Build optimized production bundle
npm run build

# Start production server
npm run start
```

## 📁 Project Structure

```
docs/
├── app/                    # Next.js app directory
│   ├── globals.css        # Global styles and CSS variables
│   ├── layout.tsx         # Root layout with theme provider
│   └── page.tsx           # Homepage with hero and features
├── components/            # React components
│   ├── ui/               # Base UI components (buttons, badges, etc.)
│   ├── hero-section.tsx  # Homepage hero with interactive background
│   ├── code-visualizer.tsx # Interactive code editor component
│   ├── features-section.tsx # Feature showcase
│   └── ...               # Additional page sections
├── lib/                  # Utility functions
│   └── utils.ts         # Common utilities and accessibility helpers
├── hooks/               # Custom React hooks
│   └── use-toast.tsx    # Toast notification system
├── public/              # Static assets
│   ├── manifest.json    # PWA manifest
│   └── sw.js           # Service worker for offline support
└── ...                 # Configuration files
```

## 🎨 Design System

### Color Palette
The platform uses a carefully crafted color system based on the visual storytelling strategy:

```css
/* Brand Colors */
--primary: #2E86AB     /* Trust, stability for core concepts */
--secondary: #A23B72   /* Creativity for advanced features */
--accent: #F18F01      /* Energy for interactive elements */

/* Semantic Colors */
--execution: #4CAF50   /* Active code execution */
--error: #F44336       /* Error states */
--warning: #FF9800     /* Warnings */
--info: #2196F3        /* Information */
```

### Typography
- **Display**: Inter font for headings and UI
- **Code**: JetBrains Mono for code blocks and technical content
- **Responsive**: Fluid typography using `clamp()` for optimal readability

### Accessibility
- WCAG 2.1 AAA compliance
- Keyboard navigation support
- Screen reader optimization
- High contrast mode
- Reduced motion support
- Focus management

## 🧪 Interactive Components

### Code Visualizer
The `CodeVisualizer` component provides:
- Syntax highlighting for GAL language
- Step-by-step execution visualization
- Variable state inspection
- Performance metrics display
- Copy-to-clipboard functionality

```tsx
<CodeVisualizer
  code={galCode}
  language="gal"
  isPlaying={true}
  showExecutionFlow={true}
  showMetrics={true}
/>
```

### Interactive Background
Particle system with:
- Mouse interaction effects
- Smooth animations
- Performance-optimized rendering
- Accessibility considerations

## 📱 Progressive Web App

The platform includes full PWA support:

- **Service Worker**: Offline documentation access
- **Web App Manifest**: Native app-like experience
- **Push Notifications**: Update alerts (optional)
- **Background Sync**: Analytics and user data sync

## 🚀 Performance Optimizations

### Core Web Vitals Targets
- **First Contentful Paint**: < 1.5s
- **Largest Contentful Paint**: < 2.5s  
- **Cumulative Layout Shift**: < 0.1
- **First Input Delay**: < 100ms

### Optimization Techniques
- **Code Splitting**: Dynamic imports for large components
- **Image Optimization**: Next.js Image component with WebP/AVIF
- **Font Loading**: Preload critical fonts, swap for performance
- **Bundle Analysis**: Webpack bundle analyzer integration
- **Service Worker**: Intelligent caching strategies

## 🧪 Testing

```bash
# Run unit tests
npm test

# Run tests in watch mode  
npm run test:watch

# Run end-to-end tests
npm run test:e2e

# Accessibility testing
npm run accessibility

# Performance testing
npm run lighthouse
```

## 🤝 Contributing

We welcome contributions to make GAL documentation even better!

### Development Workflow
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes following our coding standards
4. Test your changes thoroughly
5. Submit a pull request

### Coding Standards
- **TypeScript**: Strict mode enabled
- **ESLint**: Code quality enforcement
- **Prettier**: Consistent formatting
- **Accessibility**: Follow WCAG 2.1 AAA guidelines
- **Performance**: Consider Core Web Vitals impact

## 📊 Analytics & Monitoring

### Performance Monitoring
- Core Web Vitals tracking
- Bundle size monitoring  
- Runtime performance metrics
- User experience analytics

### Accessibility Monitoring
- Screen reader compatibility testing
- Keyboard navigation testing
- Color contrast validation
- Focus management verification

## 🔧 Configuration

### Environment Variables
```bash
NEXT_PUBLIC_GA_ID=your-google-analytics-id
NEXT_PUBLIC_APP_URL=https://docs.gal-lang.org
```

### Customization
- **Theme Colors**: Modify `tailwind.config.js`
- **Fonts**: Update `app/layout.tsx`  
- **Components**: Extend `components/ui/`
- **Content**: Edit page components

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Framer Motion**: Animation library
- **Radix UI**: Accessible component primitives  
- **Tailwind CSS**: Utility-first CSS framework
- **Monaco Editor**: Code editing experience
- **Next.js**: React framework

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/geeknik/gal/issues)
- **Discussions**: [GitHub Discussions](https://github.com/geeknik/gal/discussions)

---

Built with ❤️ by the GAL Team | Setting new standards in programming language documentation
