import '../styles/globals.css';
import Head from 'next/head';

export default function App({ Component, pageProps }) {
  return (
    <>
      <Head>
        <title>Persica Programming Language | Official Documentation</title>
        <meta name="description" content="Official documentation for Persica - Statically-typed, zero-dependency programming language built in Rust." />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link rel="icon" href="/Persica/icons/persica.png" />
      </Head>
      <Component {...pageProps} />
    </>
  );
}
