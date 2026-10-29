import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';
import { Download, CheckCircle, Terminal, AlertCircle } from 'lucide-react';

export default function InstallationPageFa() {
  const meta = {
    title: 'راهنمای نصب و راه‌اندازی',
    category: 'شروع کار',
    description: 'تمامی مراحل مورد نیاز برای نصب کامپایلر پرسیکا روی ویندوز، لینوکس و مک‌اواس.',
  };

  const toc = [
    { title: 'نصاب گرافیکی ویندوز (پیشنهادی)', id: 'windows-installer', level: 2 },
    { title: 'تأیید صحت نصب در ترمینال', id: 'verify', level: 2 },
    { title: 'کامپایل سورس با Cargo و Rust', id: 'source', level: 2 },
    { title: 'نصب افزونه رسمی VS Code', id: 'vscode-install', level: 2 },
    { title: 'عیب‌یابی متغیر PATH', id: 'troubleshooting', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="windows-installer" className="flex items-center gap-2 text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        <Download className="w-6 h-6 text-emerald-400" />
        نصاب گرافیکی ویندوز (پیشنهادی)
      </h2>
      <p>
        سریع‌ترین و بی‌دردسرترین روش برای نصب پرسیکا روی سیستم‌عامل‌های ۶۴ بیتی ویندوز، استفاده از نصاب رسمی ۱-کلیک است.
      </p>

      <div className="my-6 p-5 rounded-2xl border border-emerald-500/30 bg-emerald-500/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div>
          <h4 className="text-emerald-300 font-bold text-sm">دانلود نصاب رسمی ویندوز</h4>
          <p className="text-xs text-slate-300 mt-1">نسخه ۰.۱۳.۰ (x64) • فوق‌العاده کم‌حجم ۲.۴۵ مگابایت • بدون هیچ پیش‌نیاز یا وابستگی جانبی</p>
        </div>
        <a
          href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/Persica-0.13.0-Windows-x64-Setup.exe"
          className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-emerald-500 hover:bg-emerald-600 text-white font-bold text-xs shadow-lg shadow-emerald-500/30 transition transform hover:-translate-y-0.5 shrink-0"
        >
          <Download className="w-4 h-4" />
          <span>دانلود Setup.exe</span>
        </a>
      </div>

      <p className="text-sm font-semibold text-slate-200">آنچه نصاب ویندوز به شکل خودکار انجام می‌دهد:</p>
      <ul className="space-y-2 my-3 text-xs text-slate-300 list-disc pr-5">
        <li><strong>تنظیم خودکار PATH:</strong> مسیر کامپایلر را به متغیر محیطی PATH اضافه می‌کند تا بتوانید در هر پوشه‌ای در PowerShell یا CMD دستور <code className="text-emerald-300">persica</code> را فراخوانی کنید.</li>
        <li><strong>نصب در سطح کاربر یا مدیر:</strong> هم بدون نیاز به دسترسی Administrator برای کاربر جاری و هم با دسترسی ادمین برای تمام کاربران سیستم قابل نصب است.</li>
        <li><strong>رجیستر پسوند فایل:</strong> پسوند فایل‌های <code className="text-emerald-300">.persi</code> را در رجیستری ویندوز ثبت می‌کند.</li>
        <li><strong>حذف‌کننده یکپارچه (Uninstaller):</strong> در بخش برنامه‌های ویندوز (Add/Remove Programs) ثبت می‌شود تا در صورت تمایل با یک کلیک و تمیز کردن مسیر PATH حذف گردد.</li>
      </ul>

      <h2 id="verify" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        تأیید صحت نصب در ترمینال
      </h2>
      <p>
        پس از پایان مراحل ویزارد نصب، یک پنجره ترمینال جدید (PowerShell یا CMD) باز کرده و دستور زیر را تایپ کنید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`persica --version`}
          output={`persica 0.13.0`}
        />
      </div>

      <p>
        همچنین می‌توانید یک عبارت آزمایشی را با فلگ <code className="text-emerald-400">--eval</code> فوراً در خط فرمان اجرا کنید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`persica --eval 'print("درود بر پرسیکا! 🌿");'`}
          output={`درود بر پرسیکا! 🌿`}
        />
      </div>

      <h2 id="source" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        کامپایل سورس با Cargo و Rust
      </h2>
      <p>
        اگر روی لینوکس، مک‌اواس یا توزیع‌های دیگر هستید یا می‌خواهید کد منبع را مستقیماً کامپایل کنید، ابزار کارگوی زبان Rust تمام کار را انجام می‌دهد:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`# ۱. کلون کردن مخزن گیت
git clone https://github.com/taha-heidaryfard/Persica.git
cd Persica

# ۲. اجرای تمامی تست‌های پروژه برای اطمینان از صحت محیط
cargo test --all-targets

# ۳. کامپایل نسخه بهینه‌شده ریلیز
cargo build --release`}
          output={`Finished \`release\` profile [optimized] target(s) in 11.2s`}
        />
      </div>

      <p>
        فایل اجرایی کامپایل‌شده در مسیر زیر قرار خواهد گرفت:
      </p>
      <ul className="text-xs text-slate-300 list-disc pr-5 space-y-1 my-2">
        <li>ویندوز: <code className="text-emerald-300">target\release\persica.exe</code></li>
        <li>لینوکس / مک‌اواس: <code className="text-emerald-300">target/release/persica</code></li>
      </ul>

      <h2 id="vscode-install" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        نصب افزونه رسمی VS Code
      </h2>
      <p>
        برای تجربه برنامه‌نویسی ایده‌آل با هایلایت سینتکس، پیشنهاد خودکار کدها (IntelliSense)، بررسی خطاها هنگام ذخیره و راهنمای هاور، افزونه رسمی پرسیکا را نصب کنید:
      </p>

      <div className="my-4">
        <a
          href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/persica-0.13.0.vsix"
          className="inline-flex items-center gap-2 px-4 py-2 rounded-xl border border-slate-700 bg-slate-900 hover:bg-slate-800 text-xs font-semibold text-slate-200 transition"
        >
          <Download className="w-4 h-4 text-emerald-400" />
          <span>دانلود فایل persica-0.13.0.vsix</span>
        </a>
      </div>

      <p className="text-xs text-slate-400">نصب از طریق خط فرمان:</p>
      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`code --install-extension persica-0.13.0.vsix`}
          output={`Extension 'persica-0.13.0.vsix' was successfully installed.`}
        />
      </div>

      <h2 id="troubleshooting" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        عیب‌یابی و متغیر PATH
      </h2>
      <div className="p-4 rounded-xl border border-amber-500/30 bg-amber-500/10 text-xs text-amber-200 space-y-2">
        <div className="flex items-center gap-2 font-bold text-amber-300">
          <AlertCircle className="w-4 h-4" />
          <span>پیام 'persica' is not recognized در ترمینال؟</span>
        </div>
        <p>
          اگر ویندوز پیام داد که دستور پرسیکا شناخته نشد، کافی است ترمینال خود را ببندید و یک ترمینال تازه باز کنید تا متغیرهای محیطی سیستم نوسازی شوند. در صورت لزوم می‌توانید مسیر پوشه باینری را به شکل دستی در Environment Variables سیستم ثبت کنید.
        </p>
      </div>
    </DocsLayout>
  );
}
