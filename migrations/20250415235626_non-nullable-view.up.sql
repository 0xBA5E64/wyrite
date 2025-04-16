-- Add up migration script here
ALTER VIEW post_view RENAME uuid TO "uuid!";
ALTER VIEW post_view RENAME title TO "title!";
ALTER VIEW post_view RENAME body TO "body!";
ALTER VIEW post_view RENAME is_published TO "is_published!";
ALTER VIEW post_view RENAME date_created TO "date_created!";