(function() {var implementors = {};
implementors["hyper"] = [{"text":"impl <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"hyper/upgrade/struct.Upgraded.html\" title=\"struct hyper::upgrade::Upgraded\">Upgraded</a>","synthetic":false,"types":["hyper::upgrade::Upgraded"]},{"text":"impl <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"hyper/server/conn/struct.AddrStream.html\" title=\"struct hyper::server::conn::AddrStream\">AddrStream</a>","synthetic":false,"types":["hyper::server::tcp::addr_stream::AddrStream"]}];
implementors["salvo_core"] = [{"text":"impl <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"salvo_core/listener/unix/struct.UnixStream.html\" title=\"struct salvo_core::listener::unix::UnixStream\">UnixStream</a>","synthetic":false,"types":["salvo_core::listener::unix::UnixStream"]},{"text":"impl&lt;A, B&gt; <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a> for <a class=\"enum\" href=\"salvo_core/listener/enum.JoinedStream.html\" title=\"enum salvo_core::listener::JoinedStream\">JoinedStream</a>&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + 'static,&nbsp;</span>","synthetic":false,"types":["salvo_core::listener::JoinedStream"]}];
implementors["tokio"] = [];
implementors["tokio_util"] = [{"text":"impl&lt;L, R&gt; <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a> for <a class=\"enum\" href=\"tokio_util/either/enum.Either.html\" title=\"enum tokio_util::either::Either\">Either</a>&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a>,&nbsp;</span>","synthetic":false,"types":["tokio_util::either::Either"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()