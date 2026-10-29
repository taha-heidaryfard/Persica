import React, { useState } from 'react';
import Link from 'next/link';
import Header from '../../components/Header';
import SearchModal from '../../components/SearchModal';
import CodeBlock from '../../components/CodeBlock';
import { 
  Download, Terminal, ArrowLeft, ShieldCheck, Zap, 
  Layers, Package, Sparkles, Heart, Bot, CheckCircle2,
  Cpu, FileCode2, ChevronLeft
} from 'lucide-react';

const CODE_EXAMPLES_FA = [
  {
    id: 'hello',
    title: 'اسکریپت بدون تشریفات',
    filename: 'hello.persi',
    code: `// بدون نیاز به تابع main یا تعریف کلاس: مثل پایتون مستقیم اجرا کنید!
print("درود بر برنامه‌نویسان پرسیکا! 🌿");

int sum = 0;
for num in range(1, 6) {
    sum += num;
    print("گام:", num, "مجموع انباشته:", sum);
}
print("مجموع نهایی:", sum);`,
    output: `درود بر برنامه‌نویسان پرسیکا! 🌿
گام: 1 مجموع انباشته: 1
گام: 2 مجموع انباشته: 3
گام: 3 مجموع انباشته: 6
گام: 4 مجموع انباشته: 10
گام: 5 مجموع انباشته: 15
مجموع نهایی: 15`
  },
  {
    id: 'oop',
    title: 'کلاس‌ها و استراکت‌ها (OOP)',
    filename: 'account.persi',
    code: `// استراکت مقداری (کپی با مقدار در حافظه)
struct Transaction {
    string type;
    int amount;
}

// کلاس ارجاعی (نگهداری وضعیت و رفتار روی Heap)
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
print("موجودی حساب:", acc.get_balance());`,
    output: `موجودی حساب: 1000`
  },
  {
    id: 'exceptions',
    title: 'مدیریت استثناها و پایداری',
    filename: 'safe_math.persi',
    code: `try {
    int denominator = 0;
    if denominator == 0 {
        throw "تقسیم بر صفر در ریاضیات تعریف نشده است!";
    }
} catch (err) {
    print("خطا مهار شد:", err);
}
print("برنامه بدون توقف یا پنیک به کار خود ادامه می‌دهد! ✨");`,
    output: `خطا مهار شد: تقسیم بر صفر در ریاضیات تعریف نشده است!
برنامه بدون توقف یا پنیک به کار خود ادامه می‌دهد! ✨`
  },
  {
    id: 'prelude',
    title: 'کالکشن‌های آماده (Auto-Prelude)',
    filename: 'collections.persi',
    code: `// Stack و Queue خودکار در تمام برنامه‌ها در دسترس هستند
Stack s = Stack();
s.push("Rust");
s.push("Persica");
s.push("Compiler");

print("عنصر بالای پشته:", s.pop());
print("اندازه فعلی پشته:", s.len());`,
    output: `عنصر بالای پشته: Compiler
اندازه فعلی پشته: 2`
  },
  {
    id: 'project',
    title: 'معماری پروژه (persica.toml)',
    filename: 'persica.toml',
    language: 'toml',
    code: `[package]
name = "my_app"
version = "0.1.0"
entry = "src/main.persi"

[project]
env_dir = ".persi_env"
source_dir = "src"

[dependencies]
# ماژول‌ها و وابستگی‌های ایزوله پروژه`,
    output: `# ساخت پروژه با دستور:
# persica new my_app
# اجرای پروژه با دستور:
# persica run`
  }
];

const FEATURES_FA = [
  {
    icon: ShieldCheck,
    title: '۱۰۰٪ پیاده‌سازی با Rust و تضمین Zero-Panic',
    description: 'کامپایلر و مفسر پرسیکا کاملاً مستقل و از پایه با راست نوشته شده‌اند. هیچ خطایی در کد کاربر باعث کرش رانتایم کامپایلر نمی‌شود.'
  },
  {
    icon: Zap,
    title: 'سادگی پایتون + ایمنی سیستم‌های ایستا',
    description: 'سینتکس روان و خوانا شبیه به پایتون، در کنار سیستم نوع‌دهی ایستا و بررسی انواع در زمان کامپایل (Static Type Checking).'
  },
  {
    icon: Package,
    title: 'سیستم پروژه و محیط ایزوله .persi_env',
    description: 'مدیریت پکیج و پیکربندی پروژه با persica.toml درست شبیه Cargo راست یا Poetry پایتون، با پشتیبانی از ماژول‌های مجزا.'
  },
  {
    icon: Layers,
    title: 'شیء‌گرایی سطح اول: Structs و Classes',
    description: 'تفکیک دقیق بین داده‌های مقداری سبک (Struct) و موجودیت‌های رفتاری ارجاعی با متدها و فیلدها (Class).'
  },
  {
    icon: Cpu,
    title: 'پیش‌درآمد غنی خودمیزبان (Prelude)',
    description: 'ساختارهای داده پیشرفته نظیر Stack، Queue، Set، StringBuilder، Result و Option به صورت خودمیزبان و بدون نیاز به ایمپورت.'
  },
  {
    icon: Sparkles,
    title: 'افزونه رسمی وی‌اس‌کد با اینتلیسنس',
    description: 'پشتیبانی کامل در ادیتور VS Code با هایلایت سینتکس دقیق، راهنمای هاور، تشخیص خطاها و اسنیپت‌های آماده.'
  }
];

export default function PersianHomePage() {
  const [activeTab, setActiveTab] = useState('hello');
  const [isSearchOpen, setIsSearchOpen] = useState(false);

  const currentExample = CODE_EXAMPLES_FA.find(e => e.id === activeTab);

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-vazir selection:bg-emerald-500/30 selection:text-emerald-200" dir="rtl">
      <Header onOpenSearch={() => setIsSearchOpen(true)} />
      <SearchModal isOpen={isSearchOpen} onClose={() => setIsSearchOpen(false)} />

      {/* Hero Section */}
      <section className="relative overflow-hidden border-b border-slate-800/80 pt-16 pb-20 md:pt-24 md:pb-28">
        <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-emerald-900/20 via-slate-950 to-slate-950 pointer-events-none" />
        
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 relative z-10 text-center">
          
          {/* AI Banner Badge */}
          <div className="inline-flex items-center gap-2 rounded-full border border-purple-500/30 bg-purple-500/10 px-4 py-1.5 text-xs text-purple-300 mb-8 backdrop-blur shadow-inner">
            <Bot className="w-4 h-4 text-purple-400" />
            <span>این وب‌سایت با کمک هوش مصنوعی ساخته شده است • Built with AI</span>
          </div>

          <div className="inline-flex items-center gap-2 rounded-full border border-emerald-500/30 bg-emerald-500/10 px-3.5 py-1 text-xs font-semibold text-emerald-400 mb-6 mx-auto">
            <span>نسخه ۰.۱۳.۰ پایدار منتشر شد</span>
            <span className="font-sans">🌿</span>
          </div>

          <h1 className="text-4xl sm:text-6xl lg:text-7xl font-extrabold tracking-tight text-white max-w-4xl mx-auto leading-tight sm:leading-tight">
            زبان برنامه‌نویسی مدرن، ایستا و زیبا با الهام از پایتون
          </h1>

          <p className="mt-6 text-lg sm:text-xl text-slate-300 max-w-2xl mx-auto leading-relaxed">
            پرسیکا (Persica) ایمنی و کارایی بالای <strong>Rust</strong> را با خوانایی و راحتی زبان <strong>Python</strong> پیوند می‌زند. بدون هیچ وابستگی جانبی، سبک و سریع.
          </p>

          {/* CTA Buttons */}
          <div className="mt-10 flex flex-wrap items-center justify-center gap-4">
            <a
              href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/Persica-0.13.0-Windows-x64-Setup.exe"
              className="inline-flex items-center gap-2.5 rounded-xl bg-emerald-500 hover:bg-emerald-600 px-6 py-3.5 text-sm font-bold text-white shadow-xl shadow-emerald-500/25 transition transform hover:-translate-y-0.5"
            >
              <Download className="w-4 h-4" />
              <span>دانلود نصاب ویندوز (Setup.exe)</span>
            </a>

            <Link
              href="/fa/docs/installation"
              className="inline-flex items-center gap-2 rounded-xl border border-slate-700 bg-slate-900/80 hover:bg-slate-800 px-6 py-3.5 text-sm font-semibold text-slate-200 transition"
            >
              <span>راهنمای شروع یادگیری</span>
              <ArrowLeft className="w-4 h-4" />
            </Link>

            <Link
              href="/fa/docs/vscode"
              className="inline-flex items-center gap-2 rounded-xl border border-slate-800 bg-slate-900/40 hover:bg-slate-800/80 px-4 py-3.5 text-sm font-semibold text-slate-300 transition"
            >
              <Sparkles className="w-4 h-4 text-emerald-400" />
              <span>افزونه VS Code</span>
            </Link>
          </div>

          {/* Reymit Support Prompt */}
          <div className="mt-8">
            <a
              href="https://reymit.ir/taxtander"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 text-xs font-semibold text-rose-400 hover:text-rose-300 bg-rose-500/10 border border-rose-500/20 px-3.5 py-1.5 rounded-full transition"
            >
              <Heart className="w-3.5 h-3.5 fill-rose-500" />
              <span>حمایت مالی از سازنده (Taxtander) در رِیمیت</span>
            </a>
          </div>

        </div>
      </section>

      {/* Interactive Code Showcase */}
      <section className="py-16 md:py-24 border-b border-slate-800/80 bg-slate-950/50">
        <div className="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-10">
            <h2 className="text-2xl sm:text-3xl font-extrabold text-white">
              کدنویسی لذت‌بخش و سینتکس دلنشین
            </h2>
            <p className="mt-2 text-sm text-slate-400">
              برنامه‌ها را به سادگی یک اسکریپت اجرا کنید، یا پروژه‌های بزرگ شیء‌گرا بسازید.
            </p>
          </div>

          {/* Example Selector Tabs */}
          <div className="flex flex-wrap items-center justify-center gap-2 mb-6">
            {CODE_EXAMPLES_FA.map((ex) => (
              <button
                key={ex.id}
                onClick={() => setActiveTab(ex.id)}
                className={`px-4 py-2 rounded-xl text-xs font-semibold transition ${
                  activeTab === ex.id
                    ? 'bg-emerald-500 text-white shadow-lg shadow-emerald-500/20'
                    : 'bg-slate-900 border border-slate-800 text-slate-300 hover:text-white hover:border-slate-700'
                }`}
              >
                {ex.title}
              </button>
            ))}
          </div>

          {/* Active Code Display */}
          <div className="text-left" dir="ltr">
            <CodeBlock
              language={currentExample.language || 'persica'}
              filename={currentExample.filename}
              code={currentExample.code}
              output={currentExample.output}
            />
          </div>
        </div>
      </section>

      {/* Feature Grid */}
      <section className="py-16 md:py-24 border-b border-slate-800/80">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-2xl sm:text-4xl font-extrabold text-white">
              چرا زبان برنامه‌نویسی پرسیکا؟
            </h2>
            <p className="mt-3 text-base text-slate-400 max-w-2xl mx-auto">
              طراحی شده از ریشه برای برنامه‌نویسانی که عاشق تمیزی کد و اطمینان از صحت کارکرد هستند.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {FEATURES_FA.map((feat, idx) => {
              const Icon = feat.icon;
              return (
                <div
                  key={idx}
                  className="rounded-2xl border border-slate-800 bg-slate-900/50 p-6 hover:border-slate-700 hover:bg-slate-900/80 transition group shadow-sm"
                >
                  <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-emerald-500/10 text-emerald-400 group-hover:bg-emerald-500 group-hover:text-white transition-colors">
                    <Icon className="w-5 h-5" />
                  </div>
                  <h3 className="mt-4 text-base font-bold text-white group-hover:text-emerald-400 transition-colors">
                    {feat.title}
                  </h3>
                  <p className="mt-2 text-xs text-slate-400 leading-relaxed">
                    {feat.description}
                  </p>
                </div>
              );
            })}
          </div>
        </div>
      </section>

      {/* Documentation Quick Links */}
      <section className="py-16 border-b border-slate-800/80 bg-slate-900/30">
        <div className="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
          <h2 className="text-xl sm:text-2xl font-bold text-white mb-6 text-center">
            فهرست کامل بخش‌های مستندات
          </h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 text-xs">
            <Link href="/fa/docs/installation" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۱. نصب و راه‌اندازی</div>
                <div className="text-[11px] text-slate-400 mt-0.5">نصاب ویندوز، مسیر PATH، کارگو</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/quickstart" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۲. شروع سریع</div>
                <div className="text-[11px] text-slate-400 mt-0.5">اولین برنامه، اسکریپت تکی و فیبوناچی</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/project-system" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۳. معماری پروژه</div>
                <div className="text-[11px] text-slate-400 mt-0.5">پیکربندی persica.toml و .persi_env</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/syntax-basics" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۴. متغیرها و انواع داده</div>
                <div className="text-[11px] text-slate-400 mt-0.5">int, float, string, bool, final</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/control-flow" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۵. شروط و حلقه‌ها</div>
                <div className="text-[11px] text-slate-400 mt-0.5">if / else, for in, range, while</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/functions" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۶. توابع و بازگشت</div>
                <div className="text-[11px] text-slate-400 mt-0.5">توابع تایپ‌شده، void و بازگشتی</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/collections" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۷. آرایه‌ها و دیکشنری</div>
                <div className="text-[11px] text-slate-400 mt-0.5">آرایه‌های چندبعدی، map و متدها</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/oop" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۸. شیء‌گرایی (OOP)</div>
                <div className="text-[11px] text-slate-400 mt-0.5">مقایسه struct و class، ارجاع و مقدار</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/exceptions" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۹. مدیریت استثناها</div>
                <div className="text-[11px] text-slate-400 mt-0.5">try / catch / throw و پایداری رانتایم</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/modules" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۱۰. سیستم ماژول‌ها</div>
                <div className="text-[11px] text-slate-400 mt-0.5">ایمپورت پایتونی، پکیج‌ها و mod.persi</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/stdlib" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۱۱. کتابخانه استاندارد</div>
                <div className="text-[11px] text-slate-400 mt-0.5">Stack, Queue, Set, Result, Option</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>

            <Link href="/fa/docs/vscode" className="p-4 rounded-xl border border-slate-800 bg-slate-900/70 hover:border-emerald-500/50 hover:bg-slate-900 transition flex items-center justify-between">
              <div>
                <div className="font-bold text-slate-200">۱۲. افزونه VS Code</div>
                <div className="text-[11px] text-slate-400 mt-0.5">دانلود فایل vsix، هایلایت و اسنیپت‌ها</div>
              </div>
              <ChevronLeft className="w-4 h-4 text-slate-500" />
            </Link>
          </div>
        </div>
      </section>

      {/* Sponsor Callout */}
      <section className="py-16">
        <div className="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
          <div className="rounded-3xl border border-rose-500/30 bg-gradient-to-tr from-rose-950/20 via-pink-950/10 to-slate-900 p-8 sm:p-10 shadow-2xl flex flex-col sm:flex-row items-center justify-between gap-6">
            <div className="space-y-2 text-center sm:text-start">
              <div className="inline-flex items-center gap-2 text-rose-400 font-bold text-sm">
                <Heart className="w-4 h-4 fill-rose-500 text-rose-500" />
                <span>حمایت از توسعه پرسیکا در درگاه رِیمیت</span>
              </div>
              <h3 className="text-xl sm:text-2xl font-extrabold text-white">
                پشتیبانی از پروژه و سازنده آن
              </h3>
              <p className="text-xs text-slate-400 max-w-lg leading-relaxed">
                پرسیکا حاصل صدها ساعت تحقیق و توسعه مستقل است. هرگونه حمایت مالی شما در درگاه رسمی رِیمیت مستقیماً به پیشرفت اکوسیستم پرسیکا و ابزارهای آن کمک می‌کند.
              </p>
            </div>
            <a
              href="https://reymit.ir/taxtander"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex shrink-0 items-center gap-2 rounded-xl bg-gradient-to-r from-rose-500 to-pink-600 px-6 py-3.5 text-xs font-bold text-white shadow-xl shadow-rose-500/30 hover:from-rose-600 hover:to-pink-700 transition transform hover:-translate-y-0.5"
            >
              <span>ورود به درگاه رِیمیت</span>
              <ArrowLeft className="w-4 h-4" />
            </a>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-slate-800/80 py-12 bg-slate-950 text-xs text-slate-400">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 text-center space-y-3">
          <p>
            طراحی و پیاده‌سازی شده با عشق توسط{' '}
            <a
              href="https://taha-heidaryfard.ir/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-slate-200 font-bold hover:text-emerald-400 underline decoration-emerald-500/50 underline-offset-4 transition"
              title="محمد طاها حیدری‌فرد | توسعه‌دهنده وب و نرم‌افزار"
            >
              محمد طاها حیدری‌فرد (Mohammad Taha Heidaryfard - Taxtander)
            </a>
          </p>
          <p className="text-[11px] text-slate-400">
            <a
              href="https://taha-heidaryfard.ir/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-emerald-400 hover:text-emerald-300 font-medium transition"
            >
              🌐 وب‌سایت شخصی: taha-heidaryfard.ir (توسعه‌دهنده وب و نرم‌افزار)
            </a>
            {' • '}
            کامپایلر و رانتایم پرسیکا نسخه ۰.۱۳.۰ • صد در صد مستقل با راست • تحت لایسنس MIT
          </p>
          <div className="pt-2 text-[10px] text-slate-400 flex items-center justify-center gap-2">
            <Bot className="w-3.5 h-3.5 text-purple-400" />
            <span>این وب‌سایت با کمک هوش مصنوعی ساخته شده است • Built with AI</span>
          </div>
        </div>
      </footer>
    </div>
  );
}
