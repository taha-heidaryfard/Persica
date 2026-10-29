import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';
import { FolderTree, Package, CheckCircle2, ArrowLeft } from 'lucide-react';

export default function ProjectSystemPageFa() {
  const meta = {
    title: 'معماری پروژه و محیط .persi_env',
    category: 'معماری پروژه',
    badge: 'جدید در نسخه ۰.۱۳',
    description: 'ساختاربندی پروژه‌های چند فایلی در پرسیکا، مدیریت ماژول‌ها، فایل persica.toml و محیط ایزوله پکیج‌ها.',
  };

  const toc = [
    { title: 'بررسی اجمالی و فلسفه طراحی', id: 'overview', level: 2 },
    { title: 'ساخت داربست با persica new', id: 'new-command', level: 2 },
    { title: 'ساختار فایل persica.toml', id: 'toml-schema', level: 2 },
    { title: 'محیط پکیج‌های ایزوله .persi_env', id: 'persi-env', level: 2 },
    { title: 'سلسله‌مراتب جستجوی ماژول‌ها', id: 'resolution', level: 2 },
    { title: 'دستورات پروژه در خط فرمان', id: 'commands', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="overview" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        بررسی اجمالی و فلسفه طراحی
      </h2>
      <p>
        در نسخه <strong>۰.۱۳.۰</strong> پرسیکا، یک سیستم پروژه مدرن و بدون وابستگی خارجی افزوده شده است. این سیستم به برنامه‌نویسان اجازه می‌دهد کدهای بزرگ را با ایمپورت‌های ماژولار، تست‌های خودکار و محیط‌های ایزوله پکیج به راحتی سازمان‌دهی کنند.
      </p>

      <h2 id="new-command" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ساخت داربست پروژه جدید (Scaffolding)
      </h2>
      <p>
        برای ایجاد یک ساختار کامل پروژه با تمام فایل‌های اولیه، دستور زیر را در ترمینال اجرا کنید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`persica new analytics_app
cd analytics_app`}
          output={`Created new Persica project 'analytics_app'
  - Created persica.toml
  - Created src/main.persi
  - Created .persi_env/ environment directory`}
        />
      </div>

      <p className="text-xs font-semibold text-slate-300">درخت فایل‌های تولید شده:</p>
      <div className="my-3 p-4 rounded-xl border border-slate-800 bg-slate-900/60 font-mono text-xs text-slate-300" dir="ltr">
        <div>📁 analytics_app/</div>
        <div className="pl-4">├── 📄 persica.toml &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// پیکربندی اصلی پروژه</span></div>
        <div className="pl-4">├── 📁 src/ &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// کدهای منبع برنامه</span></div>
        <div className="pl-8">└── 📄 main.persi &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// نقطه ورود اصلی اپلیکیشن</span></div>
        <div className="pl-4">└── 📁 .persi_env/ &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// پوشه پکیج‌ها و ماژول‌های محلی ایزوله</span></div>
      </div>

      <h2 id="toml-schema" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ساختار فایل persica.toml
      </h2>
      <p>
        فایل <code className="text-emerald-400">persica.toml</code> پیکربندی اصلی پروژه شماست و به فرمت استاندارد TOML نوشته می‌شود:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="toml"
          filename="persica.toml"
          code={`[package]
name = "analytics_app"
version = "0.1.0"
entry = "src/main.persi"
source = "src"

[environment]
env_dir = ".persi_env"`}
        />
      </div>

      <div className="overflow-x-auto my-4">
        <table className="w-full text-right text-xs border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">فیلد</th>
              <th className="p-3">مقدار پیش‌فرض</th>
              <th className="p-3">توضیحات</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">package.name</td>
              <td className="p-3 font-mono" dir="ltr">""</td>
              <td className="p-3">نام منحصر به فرد پروژه.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">package.version</td>
              <td className="p-3 font-mono" dir="ltr">"0.1.0"</td>
              <td className="p-3">شماره نسخه پکیج بر اساس نگارش معنایی (SemVer).</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">package.entry</td>
              <td className="p-3 font-mono" dir="ltr">"src/main.persi"</td>
              <td className="p-3">فایلی که با اجرای <code>persica run</code> اجرا می‌شود.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">package.source</td>
              <td className="p-3 font-mono" dir="ltr">"src"</td>
              <td className="p-3">ریشه جستجوی ماژول‌های داخلی پروژه.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">environment.env_dir</td>
              <td className="p-3 font-mono" dir="ltr">".persi_env"</td>
              <td className="p-3">مسیر پوشه پکیج‌های خارجی و ماژول‌های ایزوله.</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2 id="persi-env" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        محیط پکیج‌های ایزوله `.persi_env`
      </h2>
      <p>
        در توسعه مدرن، ایزوله‌سازی وابستگی‌های هر پروژه امری حیاتی است. در پایتون این کار با Virtual Environment صورت می‌گیرد، اما ونت‌اینجین‌ها فایل‌های باینری سنگین مفسر را در هر پوشه کپی می‌کنند.
      </p>
      <div className="p-4 rounded-xl border border-emerald-500/30 bg-emerald-500/10 text-xs text-slate-300 space-y-2 my-4">
        <h4 className="font-bold text-emerald-400 text-sm">طراحی سبک پرسیکا:</h4>
        <p>
          پوشه <code className="text-emerald-300 font-bold">.persi_env</code> در پرسیکا کاملاً پیکربندی‌محور است و <strong>هیچ باینری اضافه‌ای در دیسک کپی نمی‌شود.</strong> کامپایلر پرسیکا خودکار این پوشه را به مسیر لایه‌بندی ایمپورت ماژول‌ها اضافه می‌کند.
        </p>
      </div>

      <h2 id="resolution" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        سلسله‌مراتب جستجوی ماژول‌ها (Module Resolution)
      </h2>
      <p>
        هنگامی که عبارتی مانند <code className="text-emerald-400">from my_library import calculate;</code> را می‌نویسید، کامپایلر ۴ لایه زیر را به ترتیب بررسی می‌کند:
      </p>

      <ol className="my-4 space-y-3 text-xs text-slate-300 list-decimal pr-5">
        <li>
          <strong>پوشه جاری فایل:</strong> به شکل نسبی کنار فایلی که دستور را اجرا کرده (<code className="text-emerald-400">./my_library.persi</code> یا <code className="text-emerald-400">./my_library/mod.persi</code>).
        </li>
        <li>
          <strong>پوشه سورس پروژه:</strong> همان <code className="text-emerald-400">source</code> در فایل <code className="text-emerald-400">persica.toml</code> (مثلاً <code className="text-emerald-400">src/my_library.persi</code>).
        </li>
        <li>
          <strong>محیط محلی ایزوله (.persi_env):</strong> همان <code className="text-emerald-400">env_dir</code> (مثلاً <code className="text-emerald-400">.persi_env/my_library/mod.persi</code>).
        </li>
        <li>
          <strong>کتابخانه استاندارد توکار:</strong> ماژول‌های رسمی خودمیزبان پرسیکا نظیر <code className="text-emerald-400">std.math</code> و <code className="text-emerald-400">std.collections</code>.
        </li>
      </ol>

      <h2 id="commands" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        دستورات پروژه در خط فرمان
      </h2>

      <div className="space-y-6 my-4">
        <div>
          <h4 className="text-sm font-bold text-white">اجرای پروژه:</h4>
          <div dir="ltr" className="text-left">
            <CodeBlock
              language="bash"
              filename="Terminal"
              code={`persica run`}
              output={`Running project 'analytics_app' (v0.1.0)...
[خروجی برنامه]`}
            />
          </div>
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">اجرای تست‌های خودکار:</h4>
          <p className="text-xs text-slate-400 mb-2">
            تمام فایل‌های تستی <code className="text-emerald-300">*.persi</code> را در پوشه <code className="text-emerald-300">tests/</code> پیدا کرده و اجرا می‌کند:
          </p>
          <div dir="ltr" className="text-left">
            <CodeBlock
              language="bash"
              filename="Terminal"
              code={`persica test`}
              output={`Running 3 test file(s) in 'tests' directory:
  - tests/math_test.persi ... PASSED
  - tests/model_test.persi ... PASSED
  - tests/api_test.persi ... PASSED

Summary: 3 passed, 0 failed`}
            />
          </div>
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">فراخوانی مستقیم فایل کانفیگ:</h4>
          <p className="text-xs text-slate-400 mb-2">
            می‌توانید مستقیماً فایل persica.toml را به عنوان آرگومان داده و در صورت تمایل فایل اجرایی دیگری را در کانتکست همان پروژه اجرا کنید:
          </p>
          <div dir="ltr" className="text-left">
            <CodeBlock
              language="bash"
              filename="Terminal"
              code={`# اجرای نقطه ورود تعریف‌شده در کانفیگ
persica persica.toml

# اجرای اسکریپت دلخواه در محیط پروژه جاری
persica persica.toml src/benchmarks/perf.persi`}
            />
          </div>
        </div>
      </div>
    </DocsLayout>
  );
}
