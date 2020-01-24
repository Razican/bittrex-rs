initSidebarItems({"enum":[["ErrorKind","A list specifying general categories of I/O error."],["SeekFrom","Enumeration of possible methods to seek within an I/O object."]],"fn":[["copy","Creates a future which copies all the bytes from one object to another."],["copy_buf","Creates a future which copies all the bytes from one object to another."],["empty","Constructs a new handle to an empty reader."],["repeat","Creates an instance of a reader that infinitely repeats one byte."],["sink","Creates an instance of a writer which will successfully consume all data."]],"struct":[["AllowStdIo","A simple wrapper type which allows types which implement only implement `std::io::Read` or `std::io::Write` to be used in contexts which expect an `AsyncRead` or `AsyncWrite`."],["BufReader","The `BufReader` struct adds buffering to any reader."],["BufWriter","Wraps a writer and buffers its output."],["Chain","Reader for the `chain` method."],["Close","Future for the `close` method."],["Copy","Future for the [`copy()`] function."],["CopyBuf","Future for the [`copy_buf()`] function."],["Cursor","A `Cursor` wraps an in-memory buffer and provides it with a [`AsyncSeek`] implementation."],["Empty","Reader for the [`empty()`] function."],["Error","The error type for I/O operations of the [`Read`], [`Write`], [`Seek`], and associated traits."],["Flush","Future for the `flush` method."],["IoSlice","A buffer type used with `Write::write_vectored`."],["IoSliceMut","A buffer type used with `Read::read_vectored`."],["Lines","Stream for the `lines` method."],["Read","Future for the `read` method."],["ReadExact","Future for the `read_exact` method."],["ReadHalf","The readable half of an object returned from `AsyncRead::split`."],["ReadLine","Future for the `read_line` method."],["ReadToEnd","Future for the `read_to_end` method."],["ReadToString","Future for the `read_to_string` method."],["ReadUntil","Future for the `read_until` method."],["ReadVectored","Future for the `read_vectored` method."],["Repeat","Reader for the [`repeat()`] function."],["Seek","Future for the `seek` method."],["Sink","Writer for the [`sink()`] function."],["Take","Reader for the `take` method."],["Window","A owned window around an underlying buffer."],["Write","Future for the `write` method."],["WriteAll","Future for the `write_all` method."],["WriteHalf","The writable half of an object returned from `AsyncRead::split`."],["WriteVectored","Future for the `write_vectored` method."]],"trait":[["AsyncBufRead","Read bytes asynchronously."],["AsyncBufReadExt","An extension trait which adds utility methods to `AsyncBufRead` types."],["AsyncRead","Read bytes asynchronously."],["AsyncReadExt","An extension trait which adds utility methods to `AsyncRead` types."],["AsyncSeek","Seek bytes asynchronously."],["AsyncSeekExt","An extension trait which adds utility methods to `AsyncSeek` types."],["AsyncWrite","Write bytes asynchronously."],["AsyncWriteExt","An extension trait which adds utility methods to `AsyncWrite` types."]],"type":[["Result","A specialized `Result` type for I/O operations."]]});