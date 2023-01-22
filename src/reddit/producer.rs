use crate::producer::AsyncProduce;
use async_trait::async_trait;
use std::fmt::Write as _;

enum TypePrefix {
    Comment,   // t1_
    Account,   // t2_
    Link,      // t3_
    Message,   // t4_
    Subreddit, // t5_
    Award,     // t6_
}

type Fullname = String;

struct Thing {
    prefix: TypePrefix,
    uuid: String, // Base 32 encoding
}

enum Sort {
    Relevance,
    Hot,
    Top(TopPeriod),
    New,
    Comments,
}

impl Sort {
    fn to_query_string(&self) -> String {
        match self {
            Sort::Relevance => "&sort=relevance".to_owned(),
            Sort::Hot => "&sort=hot".to_owned(),
            Sort::Top(t) => {
                let t = match t {
                    TopPeriod::Hour => "hour",
                    TopPeriod::Day => "day",
                    TopPeriod::Week => "week",
                    TopPeriod::Month => "month",
                    TopPeriod::Year => "year",
                    TopPeriod::All => "all",
                };
                format!("&sort=top&t={}", t)
            }
            Sort::New => "&sort=new".to_owned(),
            Sort::Comments => "&sort=comments".to_owned(),
        }
    }
}

enum Show {
    All,
}

enum TopPeriod {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

enum ResultType {
    Sr,
    Link,
    User,
}

struct After(Fullname);
struct Before(Fullname);

struct ListingPagination {
    after: Option<After>,   // Fullname of an item to use as the anchor point
    before: Option<Before>, // Only this or after should be set
    limit: u8,              // max number of items to return
    count: Option<usize>,   // number of items already seen in this listing
    show: Option<Show>,     // whether or not to apply filters
}

impl Default for ListingPagination {
    fn default() -> Self {
        Self {
            after: Option::None,
            before: Option::None,
            limit: 25,
            count: Option::None,
            show: Default::default(),
        }
    }
}

impl ListingPagination {
    pub fn from_previous_result(after: After, count: usize) -> Self {
        ListingPagination {
            after: Option::Some(after),
            before: Option::None,
            limit: 25,
            count: Option::Some(count),
            show: Option::None,
        }
    }

    pub fn to_query_string(&self) -> String {
        let mut query_string = format!("&limit={}", self.limit);
        if let Some(After(a)) = &self.after {
            write!(&mut query_string, "&after={}", a).unwrap();
        } else if let Some(Before(b)) = &self.before {
            write!(&mut query_string, "&before={}", b).unwrap();
        }
        if let Some(c) = self.count {
            write!(&mut query_string, "&count={}", c).unwrap();
        }
        if self.show.is_some() {
            write!(&mut query_string, "&show=all").unwrap();
        }
        query_string
    }
}

struct SubredditSearch {
    subreddit: String,
    pagination: ListingPagination,
    category: String,
    include_facets: bool,
    q: String,
    restrict_rs: bool,
    sort: Sort,
    sr_detail: bool,        // Expand subreddits
    ttype: Vec<ResultType>, // Serde rename to type
}

impl SubredditSearch {
    pub fn new(subreddit: String, category: String, q: String, sort: Sort) -> Self {
        SubredditSearch {
            subreddit,
            pagination: ListingPagination::default(),
            category,
            include_facets: false, // ?
            q,
            restrict_rs: false, // ?
            sort,
            sr_detail: false,
            ttype: Vec::new(),
        }
    }

    pub fn to_query_string(&self) -> String {
        format!("/{subreddit}/search?q={q}&restrict_rs={restrict_rs}&sort={sort}&sr_detail={sr_detail}&category={category}&include_facets={include_facets}{pagination}",
            subreddit = self.subreddit,
            q = self.q,
            restrict_rs = self.restrict_rs,
            sort = self.sort.to_query_string(),
            sr_detail = self.sr_detail,
            category = self.category,
            include_facets = self.include_facets,
            pagination = self.pagination.to_query_string()
        )
    }
}

struct Post {}

#[async_trait]
impl AsyncProduce for SubredditSearch {
    type Output = Vec<Post>;

    async fn produce(&self) -> Self::Output {
        let request_url = format!(
            "https://www.reddit.com/api/v1/subreddit/search/{queryString}",
            queryString = self.to_query_string(),
        );
        todo!();
    }
}
