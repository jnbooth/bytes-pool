//! A pool for creating byte-slices and strings that can be cheaply cloned and shared across threads
//! without allocating memory. Byte-slices are shared as [`Bytes`], and strings are shared as
//! [`ByteString`]s.
//!
//! Internally, a `BytesPool` is a wrapper around a [`BytesMut`] buffer from the [`bytes`] crate.
//! It shares data by appending the data to its buffer and then splitting the buffer off with
//! [`BytesMut::split`]. This only allocates memory if the buffer needs to resize.

#![no_std]

pub use bytes::Bytes;
pub use bytestring::ByteString;

use bytes::BytesMut;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytesPool {
    inner: BytesMut,
}

impl BytesPool {
    /// Creates a new `BytesPool` with default capacity.
    ///
    /// Resulting object has unspecified capacity.
    /// This function does not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes_pool::BytesPool;
    ///
    /// let mut pool = BytesPool::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: BytesMut::new(),
        }
    }

    /// Creates a new `BytesPool` with the specified capacity.
    ///
    /// The returned `BytesPool` will be able to hold at least `capacity` bytes
    /// without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes_pool::BytesPool;
    ///
    /// let mut pool = BytesPool::with_capacity(64);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: BytesMut::with_capacity(capacity),
        }
    }

    /// Returns the number of bytes the `BytesPool` can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes_pool::BytesPool;
    ///
    /// let b = BytesPool::with_capacity(64);
    /// assert_eq!(b.capacity(), 64);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Creates an immutable slice of bytes that can be shared across threads and cheaply cloned.
    ///
    /// No allocation is performed unless the internal buffer needs to be resized to accomodate
    /// the additional bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes_pool::BytesPool;
    ///
    /// let mut pool = BytesPool::with_capacity(64);
    ///
    /// let bytes = pool.share_bytes(b"hello world");
    ///
    /// assert_eq!(bytes, &b"hello world"[..]);
    /// ```
    #[inline]
    pub fn share_bytes(&mut self, bytes: &[u8]) -> Bytes {
        self.inner.extend_from_slice(bytes);
        self.inner.split().freeze()
    }

    /// Creates an immutable string that can be shared across threads and cheaply cloned.
    ///
    /// No allocation is performed unless the internal buffer needs to be resized to accomodate
    /// the additional bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes_pool::BytesPool;
    ///
    /// let mut pool = BytesPool::with_capacity(64);
    ///
    /// let s = pool.share_str("hello world");
    ///
    /// assert_eq!(s, "hello world");
    /// ```
    #[inline]
    pub fn share_str(&mut self, s: &str) -> ByteString {
        let bytes = self.share_bytes(s.as_bytes());
        // SAFETY: `self.inner` contains only valid UTF-8.
        unsafe { ByteString::from_bytes_unchecked(bytes) }
    }

    /// Reserves capacity for at least `additional` bytes to be inserted
    /// into the given `BytesPool`.
    ///
    /// More than `additional` bytes may be reserved in order to avoid frequent
    /// reallocations. A call to `reserve` may result in an allocation.
    ///
    /// This function performs the same optimizations as [`BytesMut::reserve`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes_pool::BytesPool;
    ///
    /// let mut pool = BytesPool::with_capacity(128);
    /// let bytes = pool.share_bytes(&[0; 64][..]);
    ///
    /// assert_eq!(pool.capacity(), 64);
    ///
    /// pool.reserve(128);
    ///
    /// assert_eq!(pool.capacity(), 128);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Attempts to cheaply reclaim already allocated capacity for at least `additional` more
    /// bytes to be inserted into the given `BytesPool` and returns `true` if it succeeded.
    ///
    /// `try_reclaim` behaves exactly like [`reserve`](BytesPool::reserve), except that it never
    /// allocates new storage  and returns a `bool` indicating whether it was successful in doing
    /// so:
    ///
    /// `try_reclaim` returns false under these conditions:
    ///  - The spare capacity left is less than `additional` bytes AND
    ///  - The existing allocation cannot be reclaimed cheaply or it was less than
    ///    `additional` bytes in size
    ///
    /// Reclaiming the allocation cheaply is possible if the `BytesPool` has no outstanding
    /// references through `Bytes` or `ByteString`s which point to its underlying storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes_pool::BytesPool;
    ///
    /// let mut pool = BytesPool::with_capacity(64);
    /// assert_eq!(true, pool.try_reclaim(64));
    /// assert_eq!(64, pool.capacity());
    ///
    /// let mut bytes = pool.share_bytes(b"abcd");
    /// assert_eq!(60, pool.capacity());
    /// assert_eq!(false, pool.try_reclaim(64));
    /// // pool has capacity for 60 bytes
    /// assert_eq!(true, pool.try_reclaim(60));
    ///
    /// drop(bytes);
    ///
    /// assert_eq!(true, pool.try_reclaim(64));
    /// ```
    #[inline]
    #[must_use = "consider BytesPool::reserve if you need an infallible reservation"]
    pub fn try_reclaim(&mut self, additional: usize) -> bool {
        self.inner.try_reclaim(additional)
    }
}
