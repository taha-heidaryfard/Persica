import React, { useState } from 'react';
import Link from 'next/link';
import Header from '../components/Header';
import SearchModal from '../components/SearchModal';
import CodeBlock from '../components/CodeBlock';
import { 
  Download, Terminal, ArrowRight, ShieldCheck, Zap, 
  Layers, Package, Sparkles, Heart, Bot, CheckCircle2,
  Cpu, FileCode2, ChevronRight
} from 'lucide-react';

const CODE_EXAMPLES = [
  {
    id: 'hello',
    title: 'Instant Script',
    filename: 'hello.persi',
    code: `// Zero ceremony: run directly like Python!
print("Greetings from Persica! 🌿");

int sum = 0;
for num in range(1, 6) {
    sum += num;
    print("Step:", num, "Accumulated:", sum);
}
print("Final total:", sum);`,
    output: `Greetings from Persica! 🌿
Step: 1 Accumulated: 1
Step: 2 Accumulated: 3
Step: 3 Accumulated: 6
Step: 4 Accumulated: 10
Step: 5 Accumulated: 15
Final total: 15`
  },
  {
    id: 'oop',
    title: 'Classes & Structs',
    filename: 'account.persi',
    code: `// Value struct (copied by value)
struct Transaction {
    string type;
    int amount;
}

// Reference class (state & behavior)
class BankAccount {
    string owner;
    int balance = 0;

    void deposit(int amt) {
        this.balance += amt;
    }

    int get_balance() {
        return this.balance;
    }
}

BankAccount acc = BankAccount();
acc.deposit(1000);
print("Account balance:", acc.get_balance());`,
    output: `Account balance: 1000`
  },
  {
    id: 'exceptions',
    title: 'Robust Exceptions',
    filename: 'safe_math.persi',
    code: `try {
    int denominator = 0;
    if denominator == 0 {
        throw "Division by zero is undefined!";
    }
} catch (err) {
    print("Caught safely:", err);
}
print("Program continues without panic! ✨");`,
    output: `Caught safely: Division by zero is undefined!
Program continues without panic! ✨`
  },
  {
    id: 'prelude',
    title: 'Auto-Prelude Collections',
    filename: 'collections.persi',
    code: `// Stack and Queue are auto-available everywhere
Stack s = Stack();
s.push("Rust");
s.push("Persica");
s.push("Compiler");

print("Top element:", s.pop());
print("Current stack size:", s.len());`,
    output: `Top element: Compiler
Current stack size: 2`
  },
  {
    id: 'project',
    title: 'persica.toml',
    filename: 'persica.toml',
    language: 'toml',
    code: `[package]
name = "my_app"
version = "0.1.0"
entry = "src/main.persi"
source = "src"

[environment]
env_dir = ".persi_env"`,
    output: `$ persica run
Running project 'my_app' (v0.1.0)...`
  }
];

export default function Home() {
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const [activeTab, setActiveTab] = useState(0);

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-emerald-500/30 selection:text-emerald-200">
      <Header onOpenSearch={() => setIsSearchOpen(true)} />
      <SearchModal isOpen={isSearchOpen} onClose={() => setIsSearchOpen(false)} />

      {/* Hero Section */}
      <section className="relative overflow-hidden pt-12 pb-20 md:pt-20 md:pb-32 border-b border-slate-900">
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[400px] bg-emerald-500/10 blur-[130px] rounded-full pointer-events-none" />

        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 relative z-10 text-center">
          
          {/* AI Banner Badge */}
          <div className="inline-flex items-center gap-2 rounded-full border border-purple-500/30 bg-purple-500/10 px-4 py-1.5 text-xs text-purple-300 mb-8 backdrop-blur-md">
            <Bot className="w-3.5 h-3.5 text-purple-400" />
            <span>Built with AI • ساخته شده با هوش مصنوعی</span>
            <span className="h-3 w-px bg-purple-500/40" />
            <span className="font-semibold text-emerald-400">Persica v0.13.0</span>
          </div>

          <h1 className="text-4xl sm:text-6xl lg:text-7xl font-extrabold tracking-tight text-white max-w-4xl mx-auto leading-[1.15]">
            The <span className="bg-gradient-to-r from-emerald-400 via-teal-300 to-green-500 bg-clip-text text-transparent">ergonomics of Python</span>, the <span className="bg-gradient-to-r from-blue-400 to-indigo-300 bg-clip-text text-transparent">safety of Rust</span>.
          </h1>

          <p className="mt-6 text-base sm:text-xl text-slate-400 max-w-2xl mx-auto leading-relaxed">
            Persica is a lightweight, statically-typed, and zero-dependency programming language crafted from scratch in Rust. Featuring a dedicated project system, isolated <code className="text-emerald-300 bg-slate-900 px-1 py-0.5 rounded">.persi_env</code>, object-oriented classes, robust exceptions, and a 1-click Windows installer.
          </p>

          {/* Action CTAs */}
          <div className="mt-10 flex flex-wrap items-center justify-center gap-4">
            <a
              href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/Persica-0.13.0-Windows-x64-Setup.exe"
              className="inline-flex items-center gap-2.5 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-600 px-6 py-3.5 text-sm font-bold text-white shadow-xl shadow-emerald-500/25 hover:from-emerald-600 hover:to-teal-700 hover:shadow-emerald-500/40 transition transform hover:-translate-y-0.5"
            >
              <Download className="w-4 h-4" />
              <span>Download Windows Setup (v0.13.0)</span>
            </a>

            <Link
              href="/docs/installation"
              className="inline-flex items-center gap-2 rounded-xl border border-slate-700 bg-slate-900/80 px-6 py-3.5 text-sm font-semibold text-slate-200 hover:bg-slate-800 hover:text-white transition"
            >
              <span>Explore Docs</span>
              <ArrowRight className="w-4 h-4" />
            </Link>

            <Link
              href="/fa"
              className="inline-flex items-center gap-1.5 rounded-xl border border-teal-500/30 bg-teal-950/20 px-5 py-3.5 text-sm font-semibold font-vazir text-teal-300 hover:bg-teal-900/30 transition"
            >
              <span>مستندات فارسی</span>
            </Link>
          </div>

          {/* Quick Terminal Snippet */}
          <div className="mt-8 flex items-center justify-center">
            <div className="flex items-center gap-3 rounded-full border border-slate-800 bg-slate-900/80 px-5 py-2 font-mono text-xs text-slate-400">
              <span className="text-emerald-400">$</span>
              <span>persica script.persi</span>
              <span className="text-slate-600">|</span>
              <span className="text-emerald-400">$</span>
              <span>persica new my_app</span>
            </div>
          </div>

        </div>
      </section>

      {/* Interactive Code Playground Carousel */}
      <section className="py-16 md:py-24 bg-slate-950/60 border-b border-slate-900">
        <div className="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
          
          <div className="text-center mb-10">
            <h2 className="text-2xl sm:text-3xl font-extrabold text-white">
              Language Tour in Seconds
            </h2>
            <p className="mt-2 text-sm text-slate-400">
              Click through the tabs to explore how clean and expressive Persica code is.
            </p>
          </div>

          {/* Tab Buttons */}
          <div className="flex items-center justify-start sm:justify-center gap-2 overflow-x-auto pb-4 mb-4">
            {CODE_EXAMPLES.map((ex, idx) => (
              <button
                key={ex.id}
                onClick={() => setActiveTab(idx)}
                className={`px-4 py-2 rounded-xl text-xs font-semibold whitespace-nowrap transition-all ${
                  activeTab === idx
                    ? 'bg-emerald-500 text-white shadow-lg shadow-emerald-500/20'
                    : 'bg-slate-900 text-slate-400 hover:text-white hover:bg-slate-800'
                }`}
              >
                {ex.title}
              </button>
            ))}
          </div>

          {/* Code Viewer */}
          <div className="animate-in fade-in duration-300">
            <CodeBlock
              filename={CODE_EXAMPLES[activeTab].filename}
              language={CODE_EXAMPLES[activeTab].language || 'persica'}
              code={CODE_EXAMPLES[activeTab].code}
              output={CODE_EXAMPLES[activeTab].output}
            />
          </div>

        </div>
      </section>

      {/* Feature Grid */}
      <section className="py-20 md:py-28 bg-slate-950">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          
          <div className="text-center max-w-2xl mx-auto mb-16">
            <h2 className="text-3xl font-extrabold text-white sm:text-4xl">
              Engineered for Simplicity & Power
            </h2>
            <p className="mt-3 text-sm text-slate-400">
              Every detail is designed to give you a modern, joy-filled coding experience.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            
            <div className="rounded-2xl border border-slate-800/80 bg-slate-900/40 p-6 hover:border-emerald-500/40 transition group">
              <div className="w-10 h-10 rounded-xl bg-emerald-500/10 border border-emerald-500/30 flex items-center justify-center text-emerald-400 mb-4 group-hover:scale-110 transition-transform">
                <Zap className="w-5 h-5" />
              </div>
              <h3 className="text-base font-bold text-white mb-2">Zero Ceremony Execution</h3>
              <p className="text-xs text-slate-400 leading-relaxed">
                Run standalone scripts directly like Python without creating a project or compiling scaffolding. Just <code className="text-emerald-400">persica script.persi</code>.
              </p>
            </div>

            <div className="rounded-2xl border border-slate-800/80 bg-slate-900/40 p-6 hover:border-emerald-500/40 transition group">
              <div className="w-10 h-10 rounded-xl bg-teal-500/10 border border-teal-500/30 flex items-center justify-center text-teal-400 mb-4 group-hover:scale-110 transition-transform">
                <Package className="w-5 h-5" />
              </div>
              <h3 className="text-base font-bold text-white mb-2">Isolated .persi_env</h3>
              <p className="text-xs text-slate-400 leading-relaxed">
                Package environment system inspired by Python’s venv, but ultra-lightweight: zero binary duplication of the executable interpreter.
              </p>
            </div>

            <div className="rounded-2xl border border-slate-800/80 bg-slate-900/40 p-6 hover:border-emerald-500/40 transition group">
              <div className="w-10 h-10 rounded-xl bg-rose-500/10 border border-rose-500/30 flex items-center justify-center text-rose-400 mb-4 group-hover:scale-110 transition-transform">
                <ShieldCheck className="w-5 h-5" />
              </div>
              <h3 className="text-base font-bold text-white mb-2">Zero Rust Panics Guarantee</h3>
              <p className="text-xs text-slate-400 leading-relaxed">
                User code errors will NEVER cause a Rust panic. Full <code className="text-rose-300">try / catch / throw</code> handling with call-stack unwinding.
              </p>
            </div>

            <div className="rounded-2xl border border-slate-800/80 bg-slate-900/40 p-6 hover:border-emerald-500/40 transition group">
              <div className="w-10 h-10 rounded-xl bg-blue-500/10 border border-blue-500/30 flex items-center justify-center text-blue-400 mb-4 group-hover:scale-110 transition-transform">
                <Layers className="w-5 h-5" />
              </div>
              <h3 className="text-base font-bold text-white mb-2">Classes & Structs</h3>
              <p className="text-xs text-slate-400 leading-relaxed">
                Clear distinction: value-semantics pure data structs (<code className="text-blue-300">struct</code>) and reference-semantics object classes (<code className="text-blue-300">class</code>).
              </p>
            </div>

            <div className="rounded-2xl border border-slate-800/80 bg-slate-900/40 p-6 hover:border-emerald-500/40 transition group">
              <div className="w-10 h-10 rounded-xl bg-purple-500/10 border border-purple-500/30 flex items-center justify-center text-purple-400 mb-4 group-hover:scale-110 transition-transform">
                <Cpu className="w-5 h-5" />
              </div>
              <h3 className="text-base font-bold text-white mb-2">Self-Hosted Prelude & Stdlib</h3>
              <p className="text-xs text-slate-400 leading-relaxed">
                Dogfooding 100%: standard library and prelude (<code className="text-purple-300">Stack, Queue, Set, StringBuilder, Result, Option</code>) written directly in Persica.
              </p>
            </div>

            <div className="rounded-2xl border border-slate-800/80 bg-slate-900/40 p-6 hover:border-emerald-500/40 transition group">
              <div className="w-10 h-10 rounded-xl bg-amber-500/10 border border-amber-500/30 flex items-center justify-center text-amber-400 mb-4 group-hover:scale-110 transition-transform">
                <Sparkles className="w-5 h-5" />
              </div>
              <h3 className="text-base font-bold text-white mb-2">Official VS Code Extension</h3>
              <p className="text-xs text-slate-400 leading-relaxed">
                Rich editor extension with syntax highlighting, live type-check diagnostics on save, IntelliSense auto-complete, hover docs, and 1-click run buttons.
              </p>
            </div>

          </div>

        </div>
      </section>

      {/* Reymit Sponsor Callout */}
      <section className="py-16 bg-slate-950/80 border-t border-slate-900">
        <div className="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
          <div className="rounded-3xl border border-rose-500/30 bg-gradient-to-r from-rose-950/20 via-pink-950/20 to-slate-900/60 p-8 md:p-10 text-center shadow-2xl relative overflow-hidden">
            <Heart className="w-10 h-10 text-rose-500 fill-rose-500 mx-auto mb-4 animate-bounce" />
            <h2 className="text-2xl sm:text-3xl font-extrabold text-white">
              Support Persica Development
            </h2>
            <p className="mt-3 text-sm text-slate-300 max-w-xl mx-auto leading-relaxed">
              Persica is an independent open-source project by <strong className="text-white">Mohammad Taha Heidaryfard (Taxtander)</strong>. If you value this language and want to support ongoing research and compilers development, consider donating via Reymit.
            </p>
            <div className="mt-6 flex flex-wrap items-center justify-center gap-3">
              <a
                href="https://reymit.ir/taxtander"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2 rounded-xl bg-gradient-to-r from-rose-500 to-pink-600 px-6 py-3 text-sm font-bold text-white shadow-lg shadow-rose-500/30 hover:from-rose-600 hover:to-pink-700 transition transform hover:-translate-y-0.5"
              >
                <span>حمایت مالی از طریق رِیمیت (Reymit)</span>
                <ArrowRight className="w-4 h-4" />
              </a>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-slate-900 py-12 text-center text-xs text-slate-400">
        <div className="mx-auto max-w-7xl px-4 space-y-2">
          <p>
            Designed & developed by{' '}
            <a
              href="https://taha-heidaryfard.ir/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-slate-200 font-bold hover:text-emerald-400 underline decoration-emerald-500/50 underline-offset-4 transition"
              title="Mohammad Taha Heidaryfard | Web & Software Developer"
            >
              Mohammad Taha Heidaryfard (Taxtander)
            </a>{' '}
            • Under MIT License
          </p>
          <p className="text-[11px] text-slate-400">
            🤖 This website was built with AI • این وب‌سایت با کمک هوش مصنوعی ساخته شده است
          </p>
          <div className="flex flex-wrap items-center justify-center gap-3 pt-3 text-slate-400 text-xs">
            <a
              href="https://taha-heidaryfard.ir/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-emerald-400 hover:text-emerald-300 font-medium transition"
            >
              🌐 taha-heidaryfard.ir
            </a>
            <span>•</span>
            <a href="https://github.com/taha-heidaryfard/Persica" target="_blank" rel="noopener noreferrer" className="hover:text-white transition">GitHub</a>
            <span>•</span>
            <Link href="/docs/installation" className="hover:text-white transition">Documentation</Link>
            <span>•</span>
            <Link href="/fa" className="hover:text-white transition font-vazir">مستندات فارسی</Link>
            <span>•</span>
            <a href="https://reymit.ir/taxtander" target="_blank" rel="noopener noreferrer" className="text-rose-400 hover:text-rose-300 transition">Reymit Sponsor</a>
          </div>
        </div>
      </footer>

    </div>
  );
}
