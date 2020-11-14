use pyo3::{basic::PyObjectProtocol, prelude::*, types::PyType};
use tantivy::schema;

/// A Facet represent a point in a given hierarchy.
///
/// They are typically represented similarly to a filepath. For instance, an
/// e-commerce website could have a Facet for /electronics/tv_and_video/led_tv.
///
/// A document can be associated to any number of facets. The hierarchy
/// implicitely imply that a document belonging to a facet also belongs to the
/// ancestor of its facet. In the example above, /electronics/tv_and_video/
/// and /electronics.
#[pyclass]
#[derive(Clone)]
pub(crate) struct Facet {
    pub(crate) inner: schema::Facet,
}

#[pymethods]
impl Facet {
    /// Create a new instance of the "root facet" Equivalent to /.
    #[classmethod]
    fn root(_cls: &PyType) -> Facet {
        Facet {
            inner: schema::Facet::root(),
        }
    }

    /// Returns true if the facet is the root facet /.
    #[getter]
    fn is_root(&self) -> bool {
        self.inner.is_root()
    }

    /// Returns true if another Facet is a subfacet of this facet.
    /// Args:
    ///     other (Facet): The Facet that we should check if this facet is a
    ///         subset of.
    fn is_prefix_of(&self, other: &Facet) -> bool {
        self.inner.is_prefix_of(&other.inner)
    }

    /// Create a Facet object from a string.
    /// Args:
    ///     facet_string (str): The string that contains a facet.
    ///
    /// Returns the created Facet.
    #[classmethod]
    fn from_string(_cls: &PyType, facet_string: &str) -> Facet {
        Facet {
            inner: schema::Facet::from_text(facet_string),
        }
    }

    /// Returns the list of `segments` that forms a facet path.
    ///
    /// For instance `//europe/france` becomes `["europe", "france"]`.
    fn to_path(&self) -> Vec<&str> {
        self.inner.to_path()
    }

    /// Returns the facet string representation.
    fn to_path_str(&self) -> String {
        self.inner.to_string()
    }
}

#[pyproto]
impl PyObjectProtocol for Facet {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Facet({})", self.to_path_str()))
    }
}
