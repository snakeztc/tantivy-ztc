from tantivy import tantivy

# Declaring our schema.
schema_builder = tantivy.SchemaBuilder()
schema_builder.add_text_field("title", stored=True, index_option='score')
schema_builder.add_text_field("body", stored=True)
schema = schema_builder.build()

# Creating our index (in memory, but filesystem is available too)
index = tantivy.Index(schema)


# Adding one document.
writer = index.writer()
writer.add_document(tantivy.Document(
    title=["The Old Man and the Sea"],
    body=["""He was an old man who fished alone in a skiff in the Gulf Stream and he had gone eighty-four days now without taking a fish."""],
))
# ... and committing
writer.commit()


# Reload the index to ensure it points to the last commit.
index.reload()
searcher = index.searcher()
query = index.parse_query_with_method("fish days", 'score', ["title", "body"])

(best_score, best_doc_address) = searcher.search(query, 3).hits[0]
print(best_score, best_doc_address)
best_doc = searcher.doc(best_doc_address)
assert best_doc["title"] == ["The Old Man and the Sea"]
print(best_doc)
