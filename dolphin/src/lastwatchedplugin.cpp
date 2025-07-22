#include "lastwatchedplugin.h"

#include <KActionCollection>
#include <KFileItem>
#include <KPluginFactory>
#include <QPointer>
#include <QStringLiteral>
#include <QtContainerFwd>

K_PLUGIN_CLASS_WITH_JSON(LastWatchedPlugin, "lastwatchedplugin.json")

const QList<std::string> VIDEO_EXTENSIONS = {"mp4", "mkv", "avi", "webm", "flv", "mov", "wmv"};
const QString ACTION_MARK_WATCHED = QStringLiteral("mark-watched");
const QString ACTION_MARK_UNWATCHED = QStringLiteral("mark-unwatched");

class LastWatchedPlugin::Private
{
public:
    QString sidecar;
    QSet<QString> watched;

    QPointer<KActionCollection> contextActions;
    QStringList contextFiles;

    Private(LastWatchedPlugin *parent)
        : watched()
        , contextActions(new KActionCollection(parent))
    {
    }

    bool loadWatched();
    bool saveWatched();
    bool isWatched(const KFileItem &item);
};

LastWatchedPlugin::LastWatchedPlugin(QObject *parent, const QVariantList &args)
    : KVersionControlPlugin(parent)
    , d(new Private(this))
{
    Q_UNUSED(args);

    connect(d->contextActions.data(), &KActionCollection::actionTriggered, this, &LastWatchedPlugin::handleContextAction);
}

LastWatchedPlugin::~LastWatchedPlugin()
{
    delete d;
}

QString LastWatchedPlugin::fileName() const
{
    return QStringLiteral(".watched");
}

bool LastWatchedPlugin::beginRetrieval(const QString &directory)
{
    d->sidecar = directory + QLatin1String(".watched");
    return d->loadWatched();
}

KVersionControlPlugin::ItemVersion LastWatchedPlugin::itemVersion(const KFileItem &item) const
{
    if (d->isWatched(item))
        return ItemVersion::NormalVersion;
    return ItemVersion::UnversionedVersion;
}

void LastWatchedPlugin::endRetrieval()
{
}

QList<QAction *> LastWatchedPlugin::versionControlActions(const KFileItemList &items) const
{
    Q_UNUSED(items);

    d->contextActions->clear();
    d->contextFiles.clear();

    auto needsWatch = false;
    auto needsUnwatch = false;
    for (auto item : items) {
        if (!VIDEO_EXTENSIONS.contains(item.suffix()))
            return d->contextActions->actions();

        auto name = item.name();
        d->contextFiles.append(name);

        if (d->watched.contains(name))
            needsUnwatch = true;
        else
            needsWatch = true;
    }

    if (needsWatch) {
        QAction *markWatched = d->contextActions->addAction(ACTION_MARK_WATCHED);
        markWatched->setText(QStringLiteral("Mark Watched"));
    }

    if (needsUnwatch) {
        QAction *markUnwatched = d->contextActions->addAction(ACTION_MARK_UNWATCHED);
        markUnwatched->setText(QStringLiteral("Mark Unwatched"));
    }

    return d->contextActions->actions();
}

QList<QAction *> LastWatchedPlugin::outOfVersionControlActions(const KFileItemList &items) const
{
    Q_UNUSED(items)
    return {};
}

void LastWatchedPlugin::handleContextAction(QAction *action)
{
    auto object = action->objectName();
    if (object == ACTION_MARK_WATCHED) {
        d->loadWatched();
        for (auto item : d->contextFiles)
            d->watched.insert(item);
        d->saveWatched();
    } else if (object == ACTION_MARK_UNWATCHED) {
        d->loadWatched();
        for (auto item : d->contextFiles)
            d->watched.remove(item);
        d->saveWatched();
    }
}

bool LastWatchedPlugin::Private::loadWatched()
{
    auto file = QFile(this->sidecar);
    if (!file.open(QIODevice::ReadOnly | QIODevice::Text))
        return false;

    while (!file.atEnd()) {
        auto line = file.readLine();
        this->watched.insert(QString::fromUtf8(line.trimmed()));
    }

    file.close();
    return true;
}

bool LastWatchedPlugin::Private::saveWatched()
{
    auto file = QFile(this->sidecar);
    if (!file.open(QIODevice::WriteOnly | QIODevice::Text))
        return false;

    for (auto item : this->watched) {
        auto bytes = item.toUtf8();
        bytes.append('\n');
        file.write(bytes);
    }

    file.close();
    return true;
}

bool LastWatchedPlugin::Private::isWatched(const KFileItem &item)
{
    return VIDEO_EXTENSIONS.contains(item.suffix()) && this->watched.contains(item.name());
}

#include "lastwatchedplugin.moc"

#include "moc_lastwatchedplugin.cpp"
